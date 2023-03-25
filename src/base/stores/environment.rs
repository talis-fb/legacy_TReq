use std::collections::HashMap;

pub struct EnvironmentStore {
    // save_file: Rc<Mutex<SaveFiles>>,
    global: HashMap<String, String>,
    session: HashMap<String, String>,
}

impl EnvironmentStore {
    pub fn init() -> Self {
        Self {
            global: HashMap::new(),
            session: HashMap::new(),
        }
    }

    pub fn get_map(&self) -> &HashMap<String, String> {
        &self.global
    }
}
