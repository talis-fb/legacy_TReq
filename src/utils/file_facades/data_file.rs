use std::path::{Path, PathBuf};
use super::{FileFacade, FileUtils};

#[derive(Clone, Debug)]
pub struct DataFile {
    path: PathBuf,
}
impl FileFacade for DataFile {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    fn get_parent_path() -> PathBuf {
        FileUtils::get_data_dir().unwrap().join("data")
    }

    fn from_name(filename: String) -> Self {
        let path = Self::get_parent_path().join(filename);
        Self { path }
    }
}
