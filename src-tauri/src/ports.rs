use crate::models::Project;
use anyhow::Result;
use async_trait::async_trait;

pub struct CreateProjectData<'a> {
    pub name: &'a str,
}

#[async_trait]
pub trait ProjectRepository: Sync {
    async fn create(&self, project: CreateProjectData<'_>) -> Result<Project>;
    async fn get(&self, id: u64) -> Result<Option<Project>>;
}
