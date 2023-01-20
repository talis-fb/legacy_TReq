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

    fn from_name(filename: String) -> Self {
        let temp_file = Builder::new()
            .prefix(&filename)
            .suffix(".json")
            .rand_bytes(0)
            .tempfile()
            .unwrap();
        let path = temp_file.path().to_path_buf();
        Self { path }
    }

    fn setup() -> Result<(), String> {
        Ok(())
    }
}
