use anyhow::Result;
use std::fmt::Debug;
use std::sync::Arc;
use validator::Validate;

use crate::models::Project;
use crate::ports::{CreateProjectData, ProjectRepository};
use crate::utils::{IsSend, IsSync};

pub struct ProjectInteractor {
    project_repository: Arc<dyn ProjectRepository + Send + Sync>,
}

impl IsSync for ProjectInteractor {}
impl IsSend for ProjectInteractor {}

impl Debug for ProjectInteractor {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("ProjectInteractor.fmt not implemented")
    }
}

impl ProjectInteractor {
    pub fn new(project_repository: Arc<dyn ProjectRepository + Send + Sync>) -> Self {
        ProjectInteractor { project_repository }
    }

    pub async fn create(&self, name: &str) -> Result<Project> {
        let data = CreateProjectData { name };
        data.validate()?;

        self.project_repository.create(data).await
    }

    pub async fn list(&self) -> Result<Vec<Project>> {
        self.project_repository.list().await
    }
}
