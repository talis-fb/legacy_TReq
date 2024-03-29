use crate::base::os::handler::FileHandler;
use crate::utils::custom_types::uuid::UUID;
use std::{collections::HashMap, rc::Rc, sync::Mutex};

pub struct EnvironmentStore {
    pub session: HashMap<String, String>,

    // Persistent
    pub global: HashMap<String, String>,
    id_file: UUID,
    file_handler: Rc<Mutex<FileHandler>>,
}

impl EnvironmentStore {
    pub fn init(file_handler: Rc<Mutex<FileHandler>>) -> Result<Self, String> {
        let file_handler_cc = file_handler.lock().unwrap();
        let map_files = file_handler_cc.get_map_files_variables();

        // 'map_files' should contains only ONE element
        let (id, file) = map_files.iter().next().unwrap();
        let content = file.get_content()?;

        let s = Self {
            session: HashMap::from([
                ("lorem".to_string(), "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.".to_string()),
                ("random_uuid".to_string(), uuid::Uuid::new_v4().to_string()),
            ]),
            global: content,
            id_file: id.clone(),
            file_handler: file_handler.clone(),
        };
        Ok(s)
    }

    pub fn get_map(&self) -> HashMap<String, String> {
        let mut map = self.global.clone();
        map.extend(self.session.clone());
        map
    }

    pub fn save_globals(&self) -> Result<(), String> {
        let mut file_handler = self.file_handler.lock().unwrap();
        file_handler.save_content_variable_file(&self.id_file, self.global.clone())?;
        Ok(())
    }
}
