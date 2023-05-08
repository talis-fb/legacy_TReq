use super::configurations::view::ViewConfig;

use crate::base::os::handler::FileHandler;
use std::rc::Rc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct ConfigManager {
    pub view: Rc<Mutex<ViewConfig>>,
    pub files: Rc<Mutex<FileHandler>>,
}
impl ConfigManager {
    pub fn init(file_handler: FileHandler, view: ViewConfig) -> Self {
        Self {
            view: Rc::new(Mutex::new(view)),
            files: Rc::new(Mutex::new(file_handler)),
        }
    }
}
