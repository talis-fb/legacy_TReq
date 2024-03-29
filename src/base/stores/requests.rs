use std::{rc::Rc, sync::Mutex};

use crate::{
    base::{os::handler::FileHandler, web::request::Request},
    utils::custom_types::uuid::UUID,
};

#[derive(Clone)]
pub struct RequestStore {
    file_handler: Rc<Mutex<FileHandler>>,
    requests: Vec<(UUID, Request)>,
    current_ind: usize,
}

impl RequestStore {
    pub fn init(file_handler: Rc<Mutex<FileHandler>>) -> Self {
        let binding = file_handler.clone();
        let save_files_cc = binding.lock().unwrap();

        let map_saved_files = save_files_cc.get_map_files_request();

        let requests: Vec<(UUID, Request)> = map_saved_files
            .iter()
            .filter_map(|(id, req_file)| {
                let req_entity = req_file.get_content().ok()?;
                Some((id.clone(), req_entity))
            })
            .collect();

        Self {
            file_handler,
            requests,
            current_ind: 0,
        }
    }

    pub fn add_request(&mut self) -> usize {
        let uuid = UUID::new();
        let req = Request::default();
        self.requests.push((uuid, req));

        let last_index = self.requests.len() - 1;

        self.goto_request(last_index);
        last_index
    }

    pub fn delete_current_request(&mut self) -> Result<(), String> {
        let i = self.current_ind;
        let (uuid, _) = self.requests.get(i).unwrap();
        let uuid = uuid.clone();

        self.requests.remove(i);

        if self.requests.is_empty() {
            self.add_request();
        }

        self.current_ind = if self.requests.get(i).is_some() {
            i
        } else {
            i - 1
        };

        let mut file_handler = self.file_handler.lock().unwrap();
        if file_handler.get_map_files_request().contains_key(&uuid) {
            file_handler.delete_request_file(&uuid)?;
        }

        Ok(())
    }

    pub fn goto_request(&mut self, index: usize) -> Option<()> {
        let _key = self.requests.get(index)?;
        self.current_ind = index;
        Some(())
    }
    pub fn goto_next_request(&mut self) {
        let next_ind = self.current_ind + 1;
        if self.goto_request(next_ind).is_none() {
            self.goto_request(0);
        }
    }
    pub fn goto_prev_request(&mut self) {
        let prev_ind = (self.current_ind as i32) - 1;

        if prev_ind < 0 {
            self.goto_request(self.get_total_requests() - 1);
            return;
        }

        self.goto_request(prev_ind as usize);
    }

    pub fn get_request(&self) -> Request {
        let key = self.requests.get(self.current_ind).unwrap();
        key.1.clone()
    }

    pub fn get_request_uuid(&self) -> &UUID {
        let key = self.requests.get(self.current_ind).unwrap();
        &key.0
    }

    pub fn get_requests(&self) -> Vec<&Request> {
        self.requests.iter().map(|(_uuid, req)| req).collect()
    }

    pub fn request_ind(&self) -> usize {
        self.current_ind
    }

    pub fn get_total_requests(&self) -> usize {
        self.requests.len()
    }

    pub fn update_request(&mut self, mut request: Request) {
        let key = self.requests.get_mut(self.current_ind).unwrap();

        request.has_changed = true;
        key.1 = request;
    }

    pub fn save_current_request(&mut self) -> Result<(), String> {
        let (uuid, req) = self.requests.get_mut(self.current_ind).unwrap();

        {
            let mut file_handler = self.file_handler.lock().unwrap();

            if !file_handler.get_map_files_request().contains_key(uuid) {
                let file = file_handler
                    .file_factory
                    .as_mut()
                    .unwrap()
                    .create_request_file(uuid.clone(), req.clone())
                    .unwrap();

                let new_uuid = file_handler.add_request(file);

                // Update UUID in 'requests' vector
                *uuid = new_uuid;
            }

            file_handler.save_content_request_file(uuid, req.clone())?;
        }

        // Now mark it as saved
        req.has_changed = false;
        Ok(())
    }
}
