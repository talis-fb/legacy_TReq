use std::{collections::HashMap, rc::Rc, sync::Mutex};

use crate::{
    base::web::request::Request,
    config::configurations::{save_files::SaveFiles, Configuration, ConfigurationEditable},
    utils::custom_types::uuid::UUID,
};

#[derive(Clone)]
pub struct RequestStore {
    save_files: Rc<Mutex<SaveFiles>>,

    request_in_memory: HashMap<UUID, Request>,
    requests: Vec<UUID>,

    current_uuid: UUID,
    current_ind: usize,
}

impl RequestStore {
    pub fn init(save_files: Rc<Mutex<SaveFiles>>) -> Self {
        let save_files_cc = save_files.clone();
        let mut save_files_content =  save_files_cc.lock().unwrap();

        let mut map_saved_files = save_files_content.get_map();
        if map_saved_files.len() == 0 {
            save_files_content.set(&UUID::new(), &Request::default()).unwrap();
            map_saved_files = save_files_content.get_map();
        }

        let request_in_memory: HashMap<UUID, Request> = map_saved_files
            .iter()
            .map(|(k, v)| {
                let req = save_files_content.get_as_entity(&k).unwrap();
                (k.clone(), req.clone())
            })
            .collect();

        let keys: Vec<UUID> = request_in_memory.clone().into_keys().collect();
        let keys_clone = keys.clone();
        let first_key = keys_clone.get(0).unwrap();

        Self {
            save_files,
            request_in_memory,
            requests: keys,
            current_uuid: first_key.clone(),
            current_ind: 0,
        }
    }

    pub fn add_request(&mut self) -> usize {
        let uuid = UUID::new();
        let req = Request::default();
        self.request_in_memory.insert(uuid.clone(), req);
        self.requests.push(uuid);

        let i = self.requests.len() - 1;

        self.goto_request(i);
        i
    }

    pub fn delete_current_request(&mut self) -> Result<(), String> {
        let uuid = self.current_uuid.clone();
        let ii = self.requests.iter().position(|u| *u == uuid);
        if let Some(i) = ii {
            self.goto_next_request();
            self.request_in_memory.remove(&uuid);
            self.requests.remove(i);
            self.save_files.lock().unwrap().remove(&uuid)?;
        }
        Ok(())
    }

    pub fn goto_request(&mut self, index: usize) -> Option<()> {
        let key = self.requests.get(index)?;
        self.current_uuid = key.clone();
        self.current_ind = index;
        Some(())
    }
    pub fn goto_next_request(&mut self) -> () {
        let next_ind = self.current_ind + 1;
        if let None = self.goto_request(next_ind) {
            self.goto_request(0);
        }
    }
    pub fn goto_prev_request(&mut self) -> () {
        let prev_ind = self.current_ind - 1;
        if let None = self.goto_request(prev_ind) {
            self.goto_request(self.get_total_requests() - 1);
        }
    }

    pub fn get_request(&self) -> Request {
        let key = self.requests.get(self.current_ind).unwrap();
        self.request_in_memory.get(key).unwrap().clone()
    }
    fn get_request_mut(&mut self) -> &mut Request {
        let key = self.requests.get(self.current_ind).unwrap();
        self.request_in_memory.get_mut(key).unwrap()
    }

    pub fn get_requests(&self) -> Vec<&Request> {
        self.requests
            .iter()
            .map(|k| self.request_in_memory.get(&k).unwrap())
            .collect()
    }

    pub fn request_ind(&self) -> usize {
        self.current_ind
    }

    pub fn get_total_requests(&self) -> usize {
        self.request_in_memory.len()
    }

    pub fn update_request(&mut self, mut request: Request) -> () {
        let key = self.requests.get(self.current_ind).unwrap();
        let request_in_memory = self.request_in_memory.get_mut(key).unwrap();

        request.has_changed = true;
        *request_in_memory = request;
    }

    pub fn save_current_request(&mut self) -> Result<(), String> {
        let uuid = &self.current_uuid;
        let req = self.get_request();
        self.save_files.lock().unwrap().set(&uuid, &req)?;

        // Now mark it as saved
        let req = self.get_request_mut();
        req.has_changed = false;
        Ok(())
    }
}
