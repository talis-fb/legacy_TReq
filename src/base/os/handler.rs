use crate::base::web::request::Request;
use crate::utils::custom_types::uuid::UUID;

use super::file_facades::requests::RequestFile;
use super::file_facades::temp_edition::TempEditionfile;
use super::file_facades::variables::VariablesFile;
use super::file_facades::FileFacade;
use super::file_factory::FileFactory;
use std::collections::HashMap;
use std::path::PathBuf;

type BoxRequestFile = Box<dyn FileFacade<UUID, Request>>;
type BoxTempEditionfile = Box<dyn FileFacade<UUID, String>>;
type BoxVariablesFile = Box<dyn FileFacade<String, HashMap<String, String>>>;

#[derive(Default)]
pub struct FileHandler {
    files_request: HashMap<UUID, BoxRequestFile>,
    files_variables: HashMap<UUID, BoxVariablesFile>,
    files_temp_edition: HashMap<UUID, BoxTempEditionfile>,

    pub file_factory: Option<Box<dyn FileFactory>>
}

// -----------------
// Setup all required folder of dependencys FileFacades
// -----------------
impl FileHandler {
    pub fn setup_env_folder() -> Result<(), String> {
        RequestFile::setup()?;
        VariablesFile::setup()?;
        TempEditionfile::setup()?;
        Ok(())
    }

    pub fn set_file_factory(&mut self, file_factory: Box<dyn FileFactory>) {
        self.file_factory = Some(file_factory);
    }
}

impl FileHandler {
    pub fn get_path(&self, key: &UUID) -> Option<PathBuf> {
        if let Some(file) = self.files_request.get(key) {
            return Some(file.get_path());
        }

        if let Some(file) = self.files_variables.get(key) {
            return Some(file.get_path());
        }

        if let Some(file) = self.files_temp_edition.get(key) {
            return Some(file.get_path());
        }

        None
    }
}

// -------------------
// Getters all maps
// -------------------
impl FileHandler {
    pub fn get_map_files_request(&self) -> &HashMap<UUID, BoxRequestFile> {
        &self.files_request
    }
    pub fn get_map_files_variables(&self) -> &HashMap<UUID, BoxVariablesFile> {
        &self.files_variables
    }
    pub fn get_map_files_temp_edition(&self) -> &HashMap<UUID, BoxTempEditionfile> {
        &self.files_temp_edition
    }
}

// -----------------
// Request
// -----------------
impl FileHandler {
    fn get_request(&mut self, key: &UUID) -> Result<&mut BoxRequestFile, String> {
        self.files_request.get_mut(key).ok_or("No file".to_string())
    }

    pub fn add_request(&mut self, file: BoxRequestFile) -> UUID {
        let key = UUID::new();
        self.files_request.insert(key.clone(), file);
        key
    }

    pub fn get_content_request_file(&mut self, key: &UUID) -> Result<Request, String> {
        self.get_request(key)?.get_content()
    }

    pub fn save_content_request_file(&mut self, key: &UUID, value: Request) -> Result<(), String> {
        self.get_request(key)?.save_content(value)
    }

    pub fn delete_request_file(&mut self, key: &UUID) -> Result<(), String> {
        self.get_request(key)?.remove()?;
        self.files_request.remove(key);
        Ok(())
    }
}

// -----------------
// Variables
// -----------------
impl FileHandler {
    fn get_variables(&mut self, key: &UUID) -> Result<&mut BoxVariablesFile, String> {
        self.files_variables
            .get_mut(key)
            .ok_or("No file".to_string())
    }

    pub fn add_variables(&mut self, file: BoxVariablesFile) -> UUID {
        let key = UUID::new();
        self.files_variables.insert(key.clone(), file);
        key
    }
    pub fn get_content_variable_file(
        &mut self,
        key: &UUID,
    ) -> Result<HashMap<String, String>, String> {
        self.get_variables(key)?.get_content()
    }

    pub fn save_content_variable_file(
        &mut self,
        key: &UUID,
        value: HashMap<String, String>,
    ) -> Result<(), String> {
        self.get_variables(key)?.save_content(value)
    }

    pub fn delete_variable_file(&mut self, key: &UUID) -> Result<(), String> {
        self.get_variables(key)?.remove()?;
        self.files_variables.remove(key);
        Ok(())
    }
}

// -----------------
// Temp Files
// -----------------
impl FileHandler {
    fn get_temp_edition(&mut self, key: &UUID) -> Result<&mut BoxTempEditionfile, String> {
        self.files_temp_edition
            .get_mut(key)
            .ok_or("No file".to_string())
    }

    pub fn add_temp_edition(&mut self, file: BoxTempEditionfile) -> UUID {
        let key = UUID::new();
        self.files_temp_edition.insert(key.clone(), file);
        key
    }
    pub fn get_content_temp_file(&mut self, key: &UUID) -> Result<String, String> {
        self.get_temp_edition(key)?.get_content()
    }

    pub fn save_content_temp_file(&mut self, key: &UUID, value: String) -> Result<(), String> {
        self.get_temp_edition(key)?.save_content(value)
    }

    pub fn delete_temp_edition_file(&mut self, key: &UUID) -> Result<(), String> {
        self.get_temp_edition(key)?.remove()?;
        self.files_temp_edition.remove(key);
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn should_init_and_be_defined() {
//
//         let request = Request::default();
//
//         let request_file = RequestFile::create(UUID::new(), request).unwrap();
//         let mut handler = FileHandler::default();
//
//         let key = handler.add_request(request_file);
//         let req_file_in_handler = handler.get_content_request_file(&key).unwrap();
//
//         assert_eq!(req_file_in_handler.url, request.url);
//     }
// }
