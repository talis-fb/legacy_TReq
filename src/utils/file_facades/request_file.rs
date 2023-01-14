use std::path::{Path, PathBuf};
use super::{FileFacade, FileUtils};

#[derive(Clone, Debug)]
pub struct RequestFile {
    pub path: PathBuf,
}
impl RequestFile {
    pub fn from_path(path: PathBuf) -> Self {
        Self { path }
    }
}
impl FileFacade for RequestFile {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    fn get_parent_path() -> PathBuf {
        FileUtils::get_data_dir().unwrap().join("requests")
    }

    fn from_name(filename: String) -> Self {
        let path = Self::get_parent_path().join(filename);
        Self { path }
    }
}
