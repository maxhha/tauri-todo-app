use crate::models::{Group, Project};
use crate::result::Result;
use async_trait::async_trait;

#[derive(validator::Validate)]
pub struct CreateProjectData<'a> {
    #[validate(length(min = 3, message = "Must be at least 3 character long"))]
    pub name: &'a str,
}

#[async_trait]
pub trait ProjectRepository: Sync + Send {
    async fn create(&self, project: CreateProjectData<'_>) -> Result<Project>;
    async fn get(&self, id: u64) -> Result<Option<Project>>;
    async fn list(&self) -> Result<Vec<Project>>;
}

pub struct CreateGroupData<'a> {
    pub name: &'a str,
    pub project_id: u64,
}

#[async_trait]
pub trait GroupRepository: Sync + Send {
    async fn create(&self, group: CreateGroupData<'_>) -> Result<Group>;
    // async fn find_by_project(&self, project_id: u64) -> Result<Vec<Group>>;
}

#[cfg(test)]
pub mod repository_tests {
    use std::{collections::HashSet, sync::Arc};

    use super::*;

    #[macro_export]
    macro_rules! project_repository_test {
        ($init:expr) => {
            $crate::project_repository_test!($init, project_repo_create_one);
            $crate::project_repository_test!($init, project_repo_create_returns_unique_ids);
            $crate::project_repository_test!($init, project_repo_get_one);
            $crate::project_repository_test!($init, project_repo_get_from_empty);
            $crate::project_repository_test!($init, project_repo_list_returns_all);
        };
        ($init:expr, $name:ident) => {
            #[tokio::test]
            async fn $name() {
                let repo = std::sync::Arc::new($init);
                $crate::ports::repository_tests::$name(repo).await;
            }
        };
    }

    #[allow(dead_code)]
    pub async fn project_repo_create_one<R: ProjectRepository>(repo: Arc<R>) {
        let name = "First project";

        let result = repo
            .create(CreateProjectData { name })
            .await
            .expect("Failed to create object");

        assert_eq!(result.name, name);
    }

    #[allow(dead_code)]
    pub async fn project_repo_create_returns_unique_ids<R: ProjectRepository + 'static>(
        repo: Arc<R>,
    ) {
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

    #[allow(dead_code)]
    pub async fn project_repo_get_one<R: ProjectRepository>(repo: Arc<R>) {
        let project = repo
            .create(CreateProjectData {
                name: "Project in repository",
            })
            .await
            .expect("Failed create project");

        let project_from_repo = repo.get(project.id).await.expect("Failed to get project");

        assert_eq!(project_from_repo, Some(project));
    }

    #[allow(dead_code)]
    pub async fn project_repo_get_from_empty<R: ProjectRepository>(repo: Arc<R>) {
        let project_from_repo = repo.get(1).await.expect("Failed to get object");

        assert_eq!(project_from_repo, None);
    }

    #[allow(dead_code)]
    pub async fn project_repo_list_returns_all<R: ProjectRepository>(repo: Arc<R>) {
        let names = vec!["First", "Second", "Third"];

        for name in names.iter() {
            repo.create(CreateProjectData { name })
                .await
                .expect("Failed create project");
        }

        let projects = repo.list().await.expect("Failed list projects");
        let project_names = projects.into_iter().map(|p| p.name).collect::<Vec<_>>();

        for name in names.iter() {
            assert!(
                project_names.contains(&name.to_string()),
                "name = {:#?} not exists in project_names {:#?}",
                name,
                project_names
            );
        }
    }

    #[macro_export]
    macro_rules! group_repository_test {
        ($init:expr) => {
            $crate::group_repository_test!($init, group_repo_create_one);
        };
        ($init:expr, $name:ident) => {
            #[tokio::test]
            async fn $name() {
                let repo = std::sync::Arc::new($init);
                $crate::ports::repository_tests::$name(repo).await;
            }
        };
    }

    #[allow(dead_code)]
    pub async fn group_repo_create_one<R: GroupRepository>(repo: Arc<R>) {
        let name = "First group";
        let project_id = 12;

        let result = repo
            .create(CreateGroupData { name, project_id })
            .await
            .expect("Failed to create object");

        assert_eq!(result.name, name);
        assert_eq!(result.project_id, project_id);
    }
}
