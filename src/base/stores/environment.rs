use std::{collections::HashMap, rc::Rc, sync::Mutex};

use crate::config::configurations::global_variables_files::GlobalVariablesFiles;
use crate::config::configurations::{Configuration, ConfigurationEditable};

pub struct EnvironmentStore {
    pub global: HashMap<String, String>,
    pub session: HashMap<String, String>,

    save_files: Rc<Mutex<GlobalVariablesFiles>>,
}

impl EnvironmentStore {
    pub fn init(save_files: Rc<Mutex<GlobalVariablesFiles>>) -> Self {
        let default_global = save_files.lock().unwrap().get_as_entity(&()).unwrap();

        Self {
            global: default_global,
            session: HashMap::from([
                ("lorem".to_string(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string()),
                ("random_uuid".to_string(), uuid::Uuid::new_v4().to_string()),
            ]),
            save_files,
        }
    }

    pub fn get_map(&self) -> HashMap<String, String> {
        let mut map = self.global.clone();
        map.extend(self.session.clone());
        map
    }

    pub fn save_globals(&self) -> Result<(), String> {
        let mut save_files = self.save_files.lock().unwrap();
        save_files.set(&(), &self.global)
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
