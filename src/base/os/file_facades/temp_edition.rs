use crate::utils::custom_types::uuid::UUID;
use tempfile::Builder;

use super::FileFacade;
use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct TempEditionfile {
    pub path: PathBuf,
}

impl Drop for TempEditionfile {
    fn drop(&mut self) {
        self.remove().ok();
    }
}

impl FileFacade<UUID, String> for TempEditionfile {
    fn setup() -> Result<(), String> {
        Ok(())
    }

    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    fn get_root_path() -> PathBuf {
        Path::new("/tmp").to_path_buf()
    }

    fn create(id: UUID, _value: String) -> Result<Self, String> {
        let temp_file = Builder::new()
            .prefix(&id.value)
            .suffix(".json")
            .rand_bytes(10)
            .tempfile()
            .map_err(|e| e.to_string())?;

        let path = temp_file.path().to_path_buf();

        Ok(Self { path })
    }
}
