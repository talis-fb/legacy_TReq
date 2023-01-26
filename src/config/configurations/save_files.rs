use crate::{
    base::web::request::Request,
    utils::{
        custom_types::uuid::UUID,
        file_facades::{request_file::RequestFile, FileFacade},
    },
};
use std::collections::HashMap;

use super::{Configuration, ConfigurationEditable};

#[derive(Clone)]
pub struct SaveFiles {
    map: HashMap<UUID, RequestFile>,
}
impl Configuration<UUID, RequestFile, Request> for SaveFiles {
    fn setup_and_init() -> Result<Self, String> {
        // setup folder used to save Request Files
        RequestFile::setup()?;

        let mut all_files = HashMap::new();
        let path = RequestFile::get_parent_path();
        let paths = std::fs::read_dir(path).map_err(|e| e.to_string())?;
        for entry in paths {
            let path = entry.map_err(|e| e.to_string())?.path();
            let app_file = RequestFile::from_path(path);

            // Verify if content in File is valid
            let content_file = app_file.get_content();
            if content_file.is_err() {
                continue;
            }
            let is_valid_json: Option<Request> =
                serde_json::from_str(&content_file.unwrap()).unwrap_or(None);
            if is_valid_json.is_none() {
                continue;
            }

            all_files.insert(UUID::new(), app_file);
        }

        Ok(Self { map: all_files })
    }

    fn get_as_file(&self, key: &UUID) -> Option<&RequestFile> {
        self.get_map().get(key)
    }

    fn get_as_entity(&self, key: &UUID) -> Result<Request, String> {
        let file = self.get_as_file(key).unwrap();
        let file_content = file.get_content()?;
        let req: Request = serde_json::from_str(&file_content).map_err(|e| e.to_string())?;
        Ok(req)
    }

    fn get_map(&self) -> &HashMap<UUID, RequestFile> {
        &self.map
    }
}
impl ConfigurationEditable<UUID, RequestFile, Request> for SaveFiles {
    fn set(&mut self, key: &UUID, value: &Request) -> Result<(), String> {
        let file_in_map = self.map.get_mut(key);
        let request_str = serde_json::to_string(&value).unwrap();

        if let Some(f) = file_in_map {
            f.save_content(request_str)?;
        } else {
            let mut new_file = RequestFile::from_name(key.value.clone());
            new_file.save_content(request_str)?;
            self.map.insert(key.clone(), new_file);
        }

        Ok(())
    }
}
impl SaveFiles {
    pub fn remove(&mut self, key: &UUID) -> Result<(), String> { 
        let file_in_map = self.map.get_mut(key).unwrap();
        file_in_map.remove()
    }
}

// #[derive(Clone)]
// pub struct SaveFiles {
//     pub files_map: HashMap<UUID, RequestFile>,
// }
// impl SaveFiles {
//     pub fn init() -> Result<Self, String> {
//         let mut all_files = HashMap::new();
//
//         let path = RequestFile::get_parent_path();
//         let paths = std::fs::read_dir(path).map_err(|e| e.to_string())?;
//         for entry in paths {
//             let path = entry.map_err(|e| e.to_string())?.path();
//             let app_file = RequestFile::from_path(path);
//
//             let uuid = UUID::new();
//
//             all_files.insert(uuid, app_file);
//         }
//
//         Ok(Self {
//             files_map: all_files,
//         })
//     }
//
//     pub fn create_saved_file(&mut self, uuid: &UUID, request: &Request) -> Result<UUID, String> {
//         // Content
//         let request_str = serde_json::to_string(&request).unwrap();
//
//         // File
//         let file_path_name = uuid.value.clone();
//         let mut new_saved_file = RequestFile::from_name(file_path_name);
//         new_saved_file.save_content(request_str)?;
//
//         self.files_map.insert(uuid.clone(), new_saved_file);
//         Ok(uuid.clone())
//     }
//
//     pub fn get_map_as_requests(&self) -> HashMap<UUID, Request> {
//         let mut result: HashMap<UUID, Request> = HashMap::new();
//         for (uuid, file_app_file) in self.files_map.iter() {
//             let file_content = file_app_file.get_content().unwrap_or("{}".to_string());
//             let json: Option<Request> = serde_json::from_str(&file_content).unwrap_or(None);
//             if let Some(req) = json {
//                 result.insert(uuid.clone(), req);
//             }
//         }
//         result
//     }
//
//     pub fn save_in_file_as_request(
//         &mut self,
//         file_uuid: &UUID,
//         req: &Request,
//     ) -> Result<(), String> {
//         let mut file = self.files_map.get_mut(file_uuid);
//         if let None = file {
//             self.create_saved_file(file_uuid, req)?;
//             file = self.files_map.get_mut(file_uuid);
//         }
//
//         let content = serde_json::to_string(req).map_err(|e| e.to_string())?;
//         file.unwrap().save_content(content)?;
//
//         Ok(())
//     }
// }
