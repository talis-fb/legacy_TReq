use crate::base::web::request::Request;
use crate::utils::custom_types::uuid::UUID;

use super::file_facades::requests::RequestFile;
use super::file_facades::temp_edition::TempEditionfile;
use super::file_facades::variables::VariablesFile;
use super::file_facades::FileFacade;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::path::PathBuf;

// pub enum FileEntity {
// Request(RequestFile),
// TempEdition(TempEditionfile),
// Variables(VariablesFile),
// Request(Box<dyn FileFacade<UUID, Request>>),
// TempEdition(Box<dyn FileFacade<UUID, String>>),
// Variables(Box<dyn FileFacade<String, HashMap<String, String>>>),
// }

#[derive(Default)]
pub struct FileHandler {
    files_request: HashMap<UUID, RequestFile>,
    files_variables: HashMap<UUID, VariablesFile>,
    files_temp_edition: HashMap<UUID, TempEditionfile>,
}
impl FileHandler {
    fn get_request(&mut self, key: &UUID) -> Result<&mut RequestFile, String> {
        self.files_request.get_mut(key).ok_or("No file".to_string())
    }

    pub fn add_request(&mut self, file: RequestFile) -> UUID {
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
}

impl FileHandler {
    fn get_variables(&mut self, key: &UUID) -> Result<&mut VariablesFile, String> {
        self.files_variables
            .get_mut(key)
            .ok_or("No file".to_string())
    }

    pub fn add_variables(&mut self, file: VariablesFile) -> UUID {
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
}

impl FileHandler {
    fn get_temp_edition(&mut self, key: &UUID) -> Result<&mut TempEditionfile, String> {
        self.files_temp_edition
            .get_mut(key)
            .ok_or("No file".to_string())
    }

    pub fn add_temp_edition(&mut self, file: TempEditionfile) -> UUID {
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
}
// fn add(&mut self, file: Box<dyn Any>) -> Result<UUID, String> {
//     let key = UUID::new();
//
//     if let Some(ss) = file.downcast_ref::<RequestFile>() {
//         self.files
//             .insert(key.clone(), FileEntity::Request(ss.clone()));
//         return Ok(key);
//     }
//
//     if let Some(ss) = file.downcast_ref::<TempEditionfile>() {
//         self.files
//             .insert(key.clone(), FileEntity::TempEdition(ss.clone()));
//         return Ok(key);
//     }
//
//     if let Some(ss) = file.downcast_ref::<VariablesFile>() {
//         self.files
//             .insert(key.clone(), FileEntity::Variables(ss.clone()));
//         return Ok(key);
//     }
//
//     Err("Not a valid FileFacade".to_string())
// }

//     pub fn save_content(&mut self, key: &UUID, content: String) -> Result<(), String> {
//         let ii = self.get_file(key).unwrap();
//
//         match ii {
//             FileEntity::Request(f) => f.save_content(content),
//         }
//
//         let file = self.create_or_open(uuid)?;
//         file.save_content(content)
//     }
//
//     pub fn get_path(&mut self, key: &UUID) -> Option<PathBuf> {
//         let file = self.files.get(key)?;
//         let path = match file {
//             FileEntity::Request(f) => f.get_path(),
//             FileEntity::TempEdition(f) => f.get_path(),
//             FileEntity::Variables(f) => f.get_path(),
//         };
//         Some(path)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_init_and_be_defined() {

        let request = Request::default();

        let request_file = RequestFile::create(UUID::new(), request).unwrap();
        let mut handler = FileHandler::default();

        let key = handler.add_request(request_file);
        let req_file_in_handler = handler.get_content_request_file(&key).unwrap();

        assert_eq!(req_file_in_handler.url, request.url);
    }
}
