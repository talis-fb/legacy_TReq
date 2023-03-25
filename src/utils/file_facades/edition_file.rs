use super::FileFacade;
use std::path::{Path, PathBuf};
use tempfile::Builder;

#[derive(Clone, Debug)]
pub struct EditionFile {
    path: PathBuf,
}
impl FileFacade for EditionFile {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    fn get_parent_path() -> PathBuf {
        Path::new("/tmp").to_path_buf()
    }

    fn from_name(filename: String) -> Result<Self, String> {
        let temp_file = Builder::new()
            .prefix(&filename)
            .suffix(".json")
            .rand_bytes(10)
            .tempfile()
            .map_err(|e| e.to_string())?;

        let path = temp_file.path().to_path_buf();

        Ok(Self { path })
    }

    fn setup() -> Result<(), String> {
        Ok(())
    }
}
