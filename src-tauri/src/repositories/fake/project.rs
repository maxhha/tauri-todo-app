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
        let item = projects.get(id as usize);

        Ok(item.cloned().map(Into::into))
    }

    async fn list(&self) -> Result<Vec<models::Project>> {
        let projects = self.projects.read().await;

        Ok(projects.iter().cloned().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, sync::Arc};

    use super::*;
    use crate::ports::{CreateProjectData, ProjectRepository};

    #[tokio::test]
    async fn create_project() {
        let repo = FakeProjectRepository::new();
        let name = "First project";

        let result = repo
            .create(CreateProjectData { name })
            .await
            .expect("Failed to create object");

        assert_eq!(result.name, name);
    }

    #[tokio::test]
    async fn create_returns_unique_ids() {
        let repo = Arc::new(FakeProjectRepository::new());
        let names = vec!["First project", "Second project", "Another project"];
        let names_n = names.len();
        let mut unique_ids = HashSet::<u64>::new();

        let tasks = names
            .into_iter()
            .map(|name| {
                let repo = repo.clone();
                tokio::spawn(async move { repo.create(CreateProjectData { name }).await })
            })
            .collect::<Vec<_>>();

        for task in tasks {
            let r = task
                .await
                .expect("Failed to finish task")
                .expect("Failed to create project");
            unique_ids.insert(r.id);
        }

        assert_eq!(names_n, unique_ids.len());
    }

    #[tokio::test]
    async fn get_by_id() {
        let repo = FakeProjectRepository::new();
        let project = repo
            .create(CreateProjectData {
                name: "Project in repository",
            })
            .await
            .expect("Failed create project");

        let project_from_repo = repo.get(project.id).await.expect("Failed to get project");

        assert_eq!(project_from_repo, Some(project));
    }

    #[tokio::test]
    async fn list_returns_all_projects() {
        let repo = Arc::new(FakeProjectRepository::new());
        let names = vec!["First", "Second", "Third"];

        for name in names.iter() {
            repo.create(CreateProjectData { name })
                .await
                .expect("Failed create project");
        }

        let projects = repo.list().await.expect("Failed list projects");
        let project_names = projects.into_iter().map(|p| p.name).collect::<Vec<_>>();

        assert_eq!(names, project_names);
    }
}
