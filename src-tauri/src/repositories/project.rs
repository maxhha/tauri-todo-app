use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::ops::DerefMut;

use crate::models;
use crate::ports;
use crate::utils::{IsSend, IsSync};
use anyhow::Context;
use anyhow::Result;
use async_trait::async_trait;
use blocking::unblock;
use fs4::FileExt;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Project {
    id: u64,
    name: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
    pub is_active: bool,
    #[serde(with = "time::serde::iso8601::option")]
    archived_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ProjectFileStorageData {
    projects: Vec<Project>,
}

struct ProjectFileStorage {
    data: ProjectFileStorageData,
    f: std::fs::File,
}

impl ProjectFileStorage {
    fn open_exclusive(path: &str) -> Result<Self> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .context(format!("Failed to open {}", path))?;

        f.lock_exclusive().context("Failed to lock exclusive")?;
        let mut f = scopeguard::guard(f, |f| {
            let _ = f.unlock();
        });

        let data = if f.metadata().context("Failed to get metadata")?.len() == 0 {
            ProjectFileStorageData {
                projects: Vec::with_capacity(1),
            }
        } else {
            bson::from_reader(f.deref_mut()).context("Failed to read document")?
        };

        let f = scopeguard::ScopeGuard::into_inner(f);

        Ok(Self { data, f })
    }

    fn read_data(path: &str) -> Result<ProjectFileStorageData> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .open(path)
            .context(format!("Failed to open {}", path))?;

        f.lock_shared().context("Failed to lock shared")?;
        let mut f = scopeguard::guard(f, |f| {
            let _ = f.unlock();
        });

        let data = if f.metadata().context("Failed to get metadata")?.len() == 0 {
            ProjectFileStorageData {
                projects: Vec::with_capacity(1),
            }
        } else {
            bson::from_reader(f.deref_mut()).context("Failed to read document")?
        };

        Ok(data)
    }

    fn save(&mut self) -> Result<()> {
        self.f
            .seek(SeekFrom::Start(0))
            .context("Failed to move cursor to start of file")?;

        self.f
            .write(&bson::to_vec(&self.data).context("Failed serialize storage")?)
            .context("Failed to write storage")?;

        Ok(())
    }
}

impl Drop for ProjectFileStorage {
    fn drop(&mut self) {
        let _ = self.f.unlock();
    }
}

impl Into<models::Project> for Project {
    fn into(self) -> models::Project {
        models::Project {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
            is_active: self.is_active,
            archived_at: self.archived_at,
        }
    }
}

pub struct ProjectRepository {
    file_path: String,
}

impl IsSync for ProjectRepository {}
impl IsSend for ProjectRepository {}

impl ProjectRepository {
    pub fn new(file_path: &str) -> Self {
        ProjectRepository {
            file_path: file_path.to_string(),
        }
    }
}

#[async_trait]
impl ports::ProjectRepository for ProjectRepository {
    async fn create(&self, data: ports::CreateProjectData<'_>) -> Result<models::Project> {
        let now = OffsetDateTime::now_utc();
        let mut project = Project {
            id: 0,
            name: data.name.to_string(),
            created_at: now,
            updated_at: now,
            is_active: true,
            archived_at: None,
        };

        let file_path = self.file_path.clone();

        unblock(move || {
            let mut storage = ProjectFileStorage::open_exclusive(&file_path)
                .context("Failed to open_exclusive storage")?;

            project.id = (storage.data.projects.len() as u64) + 1;

            storage.data.projects.push(project.clone());
            storage.save().context("Failed to save storage")?;

            Ok(project.into())
        })
        .await
    }

    async fn get(&self, id: u64) -> Result<Option<models::Project>> {
        if id == 0 {
            return Ok(None);
        }
        let file_path = self.file_path.clone();

        let data: ProjectFileStorageData = unblock(move || {
            ProjectFileStorage::read_data(&file_path).context("Failed to open_shared storage")
        })
        .await?;

        let item = data.projects.get((id - 1) as usize);

        Ok(item.cloned().map(Into::into))
    }

    async fn list(&self) -> Result<Vec<models::Project>> {
        let file_path = self.file_path.clone();

        let data: ProjectFileStorageData = unblock(move || {
            ProjectFileStorage::read_data(&file_path).context("Failed to open_shared storage")
        })
        .await?;

        Ok(data.projects.iter().cloned().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_repository_test;

    struct ScopeGuard<T, F, S>(scopeguard::ScopeGuard<T, F, S>)
    where
        T: ports::ProjectRepository,
        F: FnOnce(T) + Send,
        S: scopeguard::Strategy;

    #[async_trait]
    impl<T, F, S> ports::ProjectRepository for ScopeGuard<T, F, S>
    where
        T: ports::ProjectRepository,
        F: FnOnce(T) + Send,
        S: scopeguard::Strategy,
    {
        async fn create(&self, data: ports::CreateProjectData<'_>) -> Result<models::Project> {
            self.0.create(data).await
        }

        async fn get(&self, id: u64) -> Result<Option<models::Project>> {
            self.0.get(id).await
        }

        async fn list(&self) -> Result<Vec<models::Project>> {
            self.0.list().await
        }
    }

    project_repository_test! {{
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tmp").join("Projects.bson");
        let repo = ProjectRepository::new(path.clone().to_str().expect("Invalid path"));
        ScopeGuard(scopeguard::guard(repo, |_repo| {
            std::fs::remove_file(path).expect("Failed to remove storage file");
        }))
    }}
}
