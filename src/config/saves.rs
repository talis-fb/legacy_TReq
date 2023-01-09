use crate::{
    base::web::request::Request,
    utils::config::{AppFile, APP_DATA_PATH, APP_DATA_PATH_REQUESTS, UUID},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    path::Path,
};

#[derive(Clone)]
pub struct SaveFiles {
    pub files_map: HashMap<UUID, AppFile>,
}
impl SaveFiles {
    pub fn init() -> Result<Self, String> {
        let mut all_files = HashMap::new();

        let path = Path::new(APP_DATA_PATH_REQUESTS);
        let paths = std::fs::read_dir(path).map_err(|e| e.to_string())?;
        for entry in paths {
            let path = entry.map_err(|e| e.to_string())?.path();
            let uuid = UUID::new();
            let app_file = AppFile::init(path);
            all_files.insert(uuid, app_file);
        }

        Ok(Self {
            files_map: all_files,
        })
    }

    pub fn create_saved_file(&mut self, request: &Request) -> Result<UUID, String> {
        let uuid = UUID::new();
        let uuid_value = uuid.value.clone();
        let file_path = uuid_value.as_str();

        let path_requests = Path::new(APP_DATA_PATH_REQUESTS);

        let mut new_file_path = path_requests.join(uuid_value);
        new_file_path.set_extension("json");

        let mut new_app_file = AppFile::init(new_file_path);
        new_app_file.open_or_create_file()?;
        let request_str = serde_json::to_string(&request).unwrap();

        new_app_file.save_content(request_str)?;

        self.files_map.insert(uuid.clone(), new_app_file);

        Ok(uuid.clone())
    }

    pub fn get_map_as_requests(&self) -> HashMap<UUID, Request> {
        let mut result: HashMap<UUID, Request> = HashMap::new();
        for (uuid, file_app_file) in self.files_map.iter() {
            let file_content = file_app_file.get_content().unwrap_or("{}".to_string());
            let json: Option<Request> = serde_json::from_str(&file_content).unwrap_or(None);
            if let Some(req) = json {
                result.insert(uuid.clone(), req);
            }
        }
        result
    }

    pub fn save_in_file(&mut self, file_uuid: UUID, content: String) -> Result<(), String> {
        // let mut file = self
        //     .files_map
        //     .get(&file_uuid)
        //     .expect("Failed to get file to save in it");
        //
        // file.save_content(content)?;
        Ok(())
    }
}

// #[derive(Default, Clone)]
// pub struct AppConfig {
//     global_configs: HashMap<String, String>,
//     global_vars: HashMap<String, String>,
// }
// impl AppConfig {
//     fn create_default_config_file() {}
//     fn create_default_configs() {}
//     fn create_saved_request() {}
//     fn get_all_saved_requests() {}
//
//     pub fn init() {
//         Path::file_name
//     }
// }
//
// fn oi() {
//     let uuid_file = uuid::Uuid::new_v4();
// }
