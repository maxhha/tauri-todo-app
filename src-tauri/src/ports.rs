use crate::models::Project;
use anyhow::Result;
use async_trait::async_trait;

#[derive(validator::Validate)]
pub struct CreateProjectData<'a> {
    #[validate(length(min = 3, message = "Must be at least 3 character long"))]
    pub name: &'a str,
}

#[async_trait]
pub trait ProjectRepository: Sync {
    async fn create(&self, project: CreateProjectData<'_>) -> Result<Project>;
    async fn get(&self, id: u64) -> Result<Option<Project>>;
    async fn list(&self) -> Result<Vec<Project>>;
}
