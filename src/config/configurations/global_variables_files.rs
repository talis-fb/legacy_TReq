use crate::utils::{file_facades::{data_file::DataFile, FileFacade}, files::FileUtils};
use std::collections::HashMap;

use super::{Configuration, ConfigurationEditable};

static NAME_FILE: &str = "global_variables.json";

#[derive(Clone)]
pub struct GlobalVariablesFiles {
    file_variables: DataFile,
}
impl Configuration<(), DataFile, HashMap<String, String>> for GlobalVariablesFiles {
    fn setup_and_init() -> Result<Self, String> {
        // setup folder used to save Request Files
        DataFile::setup()?;

        let path = DataFile::get_parent_path().join(NAME_FILE);

        let mut file_variables = DataFile::from_path(path);

        let content = file_variables.get_content();
        if content.is_err() {
            file_variables.save_content("{}".to_string())?;
        }
        

        Ok(Self { file_variables })
    }

    fn get_as_file<'a>(&'a self, key: &()) -> Option<&'a DataFile> {
        Some(&self.file_variables)
    }

    fn get_as_entity(&self, key: &()) -> Result<HashMap<String, String>, String> {
        let file_content = self.file_variables.get_content()?;
        serde_json::from_str(&file_content).map_err(|e| e.to_string())
    }

    fn get_map(&self) -> &HashMap<(), DataFile> {
        todo!() // Never called
    }
}
impl ConfigurationEditable<(), DataFile, HashMap<String, String>> for GlobalVariablesFiles {
    fn set(&mut self, key: &(), value: &HashMap<String, String>) -> Result<(), String> {
        let content = serde_json::to_string(&value).unwrap();
        self.file_variables.save_content(content)
    }
}
