use crate::utils::custom_types::uuid::UUID;
use tempfile::Builder;

use super::FileFacade;
use std::io::Write;
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

    fn create(id: UUID, value: String) -> Result<Self, String> {
        let mut temp_file = Builder::new()
            .prefix(&id.value)
            .suffix(".json")
            .rand_bytes(10)
            .tempfile()
            .map_err(|e| e.to_string())?;

        temp_file.write_all(value.as_bytes()).map_err(|e| e.to_string())?;

        let path = temp_file.path().to_path_buf();
        let temp_file_facade = Self { path };

        // Create file in '/tmp' folder to be used then
        temp_file.into_temp_path().keep().map_err(|e| e.to_string())?;

        Ok(temp_file_facade)
    }
}
