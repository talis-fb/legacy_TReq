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

    pub fn create_saved_file(&mut self, uuid: &UUID, request: &Request) -> Result<UUID, String> {
        let file_path = uuid.value.as_str();

        let mut new_file_path = Path::new(APP_DATA_PATH_REQUESTS).join(file_path);
        new_file_path.set_extension("json");

        let mut new_app_file = AppFile::init(new_file_path);
        let file = new_app_file.open_or_create_file()?;

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

    pub fn save_in_file_as_request(&mut self, file_uuid: &UUID, req: &Request) -> Result<(), String> {
        let mut file = self.files_map.get_mut(file_uuid);
        if let None = file {
            println!("SEM ARQUIVO");
            self.create_saved_file(file_uuid, req)?;
            file = self.files_map.get_mut(file_uuid);
        }

        let content = serde_json::to_string(req).map_err(|e| e.to_string())?;
        file.unwrap().save_content(content)?;

        Ok(())
    }
}
