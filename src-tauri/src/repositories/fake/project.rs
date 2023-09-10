use crate::models;
use crate::ports;
use crate::utils::{IsSend, IsSync};
use anyhow::Result;
use async_trait::async_trait;
use tauri::async_runtime::RwLock;
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq)]
struct Project {
    id: u64,
    name: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
    is_active: bool,
    archived_at: Option<OffsetDateTime>,
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

pub struct FakeProjectRepository {
    projects: RwLock<Vec<Project>>,
}

impl IsSync for FakeProjectRepository {}
impl IsSend for FakeProjectRepository {}

impl FakeProjectRepository {
    pub const fn new() -> Self {
        FakeProjectRepository {
            projects: RwLock::const_new(Vec::new()),
        }
    }
}

#[async_trait]
impl ports::ProjectRepository for FakeProjectRepository {
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

        let mut projects = self.projects.write().await;
        project.id = (projects.len() as u64) + 1;

        projects.push(project.clone());

        Ok(project.into())
    }

    async fn get(&self, id: u64) -> Result<Option<models::Project>> {
        let projects = self.projects.read().await;
        if id == 0 {
            return Ok(None);
        }
        let item = projects.get((id - 1) as usize);

        Ok(item.cloned().map(Into::into))
    }

    async fn list(&self) -> Result<Vec<models::Project>> {
        let projects = self.projects.read().await;

        Ok(projects.iter().cloned().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_repository_test;

    project_repository_test! {FakeProjectRepository::new()}
}
