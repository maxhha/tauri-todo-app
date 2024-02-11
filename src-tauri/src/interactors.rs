use std::fmt::Debug;
use std::sync::Arc;
use validator::Validate;

use crate::models::{Group, Project};
use crate::ports;
use crate::result::Result;
use crate::utils::{IsSend, IsSync};

pub struct ProjectInteractor {
    project_repository: Arc<dyn ports::ProjectRepository + Send + Sync>,
}

impl IsSync for ProjectInteractor {}
impl IsSend for ProjectInteractor {}

impl Debug for ProjectInteractor {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("ProjectInteractor.fmt not implemented")
    }
}

impl ProjectInteractor {
    pub fn new(project_repository: Arc<dyn ports::ProjectRepository + Send + Sync>) -> Self {
        ProjectInteractor { project_repository }
    }

    pub async fn create(&self, name: &str) -> Result<Project> {
        let data = ports::CreateProjectData { name };
        data.validate()?;

        self.project_repository.create(data).await
    }

    pub async fn list(&self) -> Result<Vec<Project>> {
        self.project_repository.list().await
    }
}

pub struct GroupInteractor {
    group_repository: Arc<dyn ports::GroupRepository + Send + Sync>,
}

impl IsSync for GroupInteractor {}
impl IsSend for GroupInteractor {}

impl Debug for GroupInteractor {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("GroupInteractor.fmt not implemented")
    }
}

impl GroupInteractor {
    pub fn new(group_repository: Arc<dyn ports::GroupRepository + Send + Sync>) -> Self {
        GroupInteractor { group_repository }
    }

    pub async fn create(&self, name: &str, project_id: u64) -> Result<Group> {
        self.group_repository
            .create(ports::CreateGroupData { name, project_id })
            .await
    }
}
