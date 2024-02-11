use crate::models;
use crate::ports;
use crate::result::Result;
use crate::utils::{IsSend, IsSync};
use async_trait::async_trait;

pub struct GroupRepository {
    // file_path: std::path::PathBuf,
}

impl IsSync for GroupRepository {}
impl IsSend for GroupRepository {}

impl GroupRepository {
    pub fn new() -> Self {
        GroupRepository {}
    }
    // pub fn new(file_path: &Path) -> Self {
    //     GroupRepository {
    //         file_path: std::path::PathBuf::from(file_path),
    //     }
    // }
}

#[async_trait]
impl ports::GroupRepository for GroupRepository {
    async fn create(&self, data: ports::CreateGroupData<'_>) -> Result<models::Group> {
        Ok(models::Group {
            id: 1,
            name: data.name.to_string(),
            position: 1,
            is_opened: true,
            project_id: data.project_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::group_repository_test;

    struct GroupRepositoryTest {
        repo: GroupRepository,
        // path: std::path::PathBuf,
    }

    // impl Drop for GroupRepositoryTest {
    //     fn drop(&mut self) {
    //         let _ = std::fs::remove_file(&self.path);
    //     }
    // }

    #[async_trait]
    impl ports::GroupRepository for GroupRepositoryTest {
        async fn create(&self, data: ports::CreateGroupData<'_>) -> Result<models::Group> {
            self.repo.create(data).await
        }
    }

    group_repository_test! {{
        // let name = format!("test_Projects_{}.bson", rand::random::<u32>());
        // let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tmp").join(name);
        GroupRepositoryTest {
            // repo: GroupRepository::new(&path),
            repo: GroupRepository::new(),
            // path,
        }
    }}
}
