use crate::utils::files::FileUtils;

use super::FileFacade;
use std::{collections::HashMap, path::PathBuf};

#[derive(Clone, Debug)]
pub struct VariablesFile {
    pub path: PathBuf,
}
impl FileFacade<String, HashMap<String, String>> for VariablesFile {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    fn get_root_path() -> PathBuf {
        FileUtils::get_data_dir().unwrap().join("data")
    }

    fn create(id: String, value: HashMap<String, String>) -> Result<Self, String> {
        let path = Self::get_root_path().join(id);
        let mut file = Self { path };
        file.save_content(value)?;
        Ok(file)
    }
}

// impl VariablesFile {
//     pub fn factory_saved_files() -> HashMap<UUID, Self> {
//         todo!()
//     }
// }
