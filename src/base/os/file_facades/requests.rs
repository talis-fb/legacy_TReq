use crate::{
    base::web::request::Request,
    utils::{custom_types::uuid::UUID, files::FileUtils},
};

use super::FileFacade;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct RequestFile {
    pub path: PathBuf,
}
impl FileFacade<UUID, Request> for RequestFile {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    fn get_root_path() -> PathBuf {
        FileUtils::get_data_dir().unwrap().join("requests")
    }

    fn create(id: UUID, value: Request) -> Result<Self, String> {
        let path = Self::get_root_path().join(id.value);
        let mut file = Self { path };
        file.save_content(value)?;
        Ok(file)
    }
}
