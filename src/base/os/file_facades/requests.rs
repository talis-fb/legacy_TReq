use crate::{
    base::web::request::Request,
    utils::{custom_types::uuid::UUID, files::FileUtils},
};

use super::FileFacade;
use std::{collections::HashMap, path::PathBuf};

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

impl RequestFile {
    pub fn factory_saved_files() -> Result<Vec<Self>, String> {
        let mut all_files = vec![];
        let root_path = RequestFile::get_root_path();
        let paths = std::fs::read_dir(root_path).map_err(|e| e.to_string())?;
        for entry in paths {
            let path = entry.map_err(|e| e.to_string())?.path();
            let request_file = RequestFile { path };

            // Verify if content in File is valid
            if request_file.get_content().is_ok() {
                all_files.push(request_file);
            }
        }

        Ok(all_files)
    }
}
