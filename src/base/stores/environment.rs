use std::collections::HashMap;

pub struct EnvironmentStore {
    pub global: HashMap<String, String>,
    pub session: HashMap<String, String>,
}

// TODO: 
// TODO: Change from HashMap to a Vec<(String, String)>, this way, order is guarated
// TODO: 

impl EnvironmentStore {
    pub fn init() -> Self {
        Self {
            global: HashMap::new(),
            session: HashMap::from([
                ("lorem".to_string(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string()),
                ("random_uuid".to_string(), "1aa05d53-711d-413f-b4ba-f9a5edd8bf2c".to_string()),
                ("random_uuid1".to_string(), "1aa05d53-711d-413f-b4ba-f9a5edd8bf2c".to_string()),
                ("random_uuid2".to_string(), "1aa05d53-711d-413f-b4ba-f9a5edd8bf2c".to_string()),
                ("random_uuid3".to_string(), "1aa05d53-711d-413f-b4ba-f9a5edd8bf2c".to_string()),
            ]),
        }
    }

    pub fn get_map(&self) -> HashMap<String, String> {
        let mut map = self.global.clone();
        map.extend(self.session.clone());
        map
    }

    // pub fn get_value(&self, key: &String) -> Option<String> {
    //     let value_in_session = self.session.get(key);
    //
    //     if value_in_session.is_some() {
    //         value_in_session.cloned()
    //     } else {
    //         self.global.get(key).cloned()
    //     }
    // }
    //
    // pub fn set_value(&mut self, key: String, value: String) {
    //      self.session.insert(key, value);
    // }
    //
    // pub fn set_value_globally(&mut self, key: String, value: String) {
    //      self.global.insert(key, value);
    // }
}
