use std::{collections::HashMap, rc::Rc, sync::Arc};

use crate::{base::web::request::Request, config::saves::SaveFiles, utils::config::UUID};

#[derive(Clone)]
pub struct RequestStore {
    save_files: SaveFiles,

    request_in_memory: HashMap<UUID, Request>,
    requests: Vec<UUID>,

    current_uuid: UUID,
    current_ind: usize,
}

impl RequestStore {
    pub fn init(save_files: SaveFiles) -> Self {
        let mut request_in_memory = save_files.get_map_as_requests();

        if request_in_memory.len() == 0 {
            request_in_memory.insert(UUID::new(), Request::default());
        }

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
            self.goto_request( self.get_total_requests() - 1 );
        }

    }

    pub fn get_request(&self) -> Request {
        let key = self.requests.get(self.current_ind).unwrap();
        self.request_in_memory.get(key).unwrap().clone()
    }

    pub fn get_requests(&self) -> Vec<&Request> {
        self.request_in_memory.values().collect()
    }

    pub fn request_ind(&self) -> usize {
        self.current_ind
    }

    pub fn get_total_requests(&self) -> usize {
        self.request_in_memory.len()
    }

    pub fn update_request(&mut self, request: Request) -> () {
        let key = self.requests.get(self.current_ind).unwrap();
        let request_in_memory = self.request_in_memory.get_mut(key).unwrap();
        *request_in_memory = request;
    }

    pub fn save_current_request(&mut self) -> Result<(), String> {
        Ok(())
        // let (uuid, req) = &self.request_in_memory[self.current];
        // let content_to_file = serde_json::to_string(req).map_err(|e| e.to_string())?;
        // self.save_files.save_in_file(uuid.clone(), content_to_file)
    }
}
