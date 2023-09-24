use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::ops::Deref;
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
    #[serde(with = "time::serde::timestamp")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::timestamp")]
    pub updated_at: OffsetDateTime,
    pub is_active: bool,
    #[serde(with = "time::serde::timestamp::option")]
    archived_at: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ProjectStorageFile {
    projects: Vec<Project>,
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
            let mut f = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&file_path)
                .context(format!("Failed to open {}", file_path))?;

            f.lock_exclusive().context("Failed to lock exclusive")?;
            let mut f = scopeguard::guard(&mut f, |f| {
                let _ = f.unlock();
            });

            let mut storage = if f.metadata().context("Failed to get metadata")?.len() == 0 {
                ProjectStorageFile {
                    projects: Vec::with_capacity(1),
                }
            } else {
                bson::from_reader(f.deref_mut()).context("Failed to read document")?
            };

            project.id = (storage.projects.len() as u64) + 1;

            storage.projects.push(project.clone());

            f.deref_mut()
                .seek(SeekFrom::Start(0))
                .context("Failed to move cursor to start of file")?;

            f.deref_mut()
                .write(&bson::to_vec(&storage).context("Failed serialize storage")?)
                .context("Failed to write storage")?;

            Ok(project.into())
        })
        .await
    }

    async fn get(&self, id: u64) -> Result<Option<models::Project>> {
        panic!("ProjectRepository::get not implemented");

        // let projects = self.projects.read().await;
        // let item = projects.get(id as usize);

        // Ok(item.cloned().map(Into::into))
    }

    async fn list(&self) -> Result<Vec<models::Project>> {
        panic!("ProjectRepository::list not implemented");

        // let projects = self.projects.read().await;

        // Ok(projects.iter().cloned().map(Into::into).collect())
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
