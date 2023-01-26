use std::{rc::Rc, sync::Mutex};

use crate::{
    base::web::request::Request,
    config::configurations::{save_files::SaveFiles, Configuration, ConfigurationEditable},
    utils::custom_types::uuid::UUID,
};

#[derive(Clone)]
pub struct RequestStore {
    save_files: Rc<Mutex<SaveFiles>>,
    requests: Vec<(UUID, Request)>,
    current_ind: usize,
}

impl RequestStore {
    pub fn init(save_files: Rc<Mutex<SaveFiles>>) -> Self {
        let save_files_cc = save_files.clone();
        let mut save_files_content = save_files_cc.lock().unwrap();

        let mut map_saved_files = save_files_content.get_map();
        if map_saved_files.is_empty() {
            save_files_content
                .set(&UUID::new(), &Request::default())
                .unwrap();
            map_saved_files = save_files_content.get_map();
        }

        let requests: Vec<(UUID, Request)> = map_saved_files
            .iter()
            .map(|(k, v)| {
                let req = save_files_content.get_as_entity(k).unwrap();
                (k.clone(), req)
            })
            .collect();

        Self {
            save_files,
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

        let mut save_files = self.save_files.lock().unwrap();
        if let Some(_) = save_files.exist(&uuid) {
            save_files.remove(&uuid)?;
        }

        Ok(())
    }

    pub fn goto_request(&mut self, index: usize) -> Option<()> {
        let key = self.requests.get(index)?;
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
        let prev_ind = self.current_ind - 1;
        if self.goto_request(prev_ind).is_none() {
            self.goto_request(self.get_total_requests() - 1);
        }
    }

    pub fn get_request(&self) -> Request {
        let key = self.requests.get(self.current_ind).unwrap();
        key.1.clone()
    }

    pub fn get_request_uuid(&self) -> &UUID {
        let key = self.requests.get(self.current_ind).unwrap();
        &key.0
    }

    fn get_request_mut(&mut self) -> &mut Request {
        let req = self.requests.get_mut(self.current_ind).unwrap();
        &mut req.1
    }

    pub fn get_requests(&self) -> Vec<&Request> {
        self.requests.iter().map(|(uuid, req)| req).collect()
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
        let (uuid, req) = self.requests.get(self.current_ind).unwrap();
        self.save_files.lock().unwrap().set(uuid, &req)?;

        // Now mark it as saved
        let req = self.get_request_mut();
        req.has_changed = false;
        Ok(())
    }
}
