use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::path::PathBuf;
use treq::base::os::file_facades::FileFacade;

pub struct MockFile<FileID, FileEntity> {
    id: FileID,
    value: FileEntity,
}

impl<FileID, FileEntity> FileFacade<FileID, FileEntity> for MockFile<FileID, FileEntity>
where
    FileID: PartialEq + Eq + Hash + 'static,
    FileEntity: for<'a> Deserialize<'a> + Serialize + Clone + 'static,
{
    fn get_root_path() -> std::path::PathBuf {
        PathBuf::new()
    }

    fn get_path(&self) -> std::path::PathBuf {
        PathBuf::new()
    }

    fn get_content(&self) -> Result<FileEntity, String> {
        Ok(self.value.clone())
    }
    fn remove(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn save_content(&mut self, value: FileEntity) -> Result<(), String> {
        self.value = value;
        Ok(())
    }

    fn setup() -> Result<(), String> {
        Ok(())
    }
    fn create(id: FileID, value: FileEntity) -> Result<Self, String> {
        Ok(Self { id, value })
    }
}
