use super::{FileFacade, FileUtils};
use std::path::PathBuf;

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

    fn from_name(filename: String) -> Result<Self, String> {
        let path = Self::get_parent_path().join(filename);
        Ok(Self { path })
    }
}
