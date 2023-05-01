use super::configurations::view::ViewConfig;
use crate::base::os::os_commands::external_editor::OsCommandEditor;
use crate::base::os::handler::FileHandler;
use std::rc::Rc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct ConfigManager {
    // pub editor: Rc<OsCommandEditor>,
    pub view: Rc<Mutex<ViewConfig>>,
    pub files: Rc<Mutex<FileHandler>>,
}
impl ConfigManager {
    pub fn init(file_handler: FileHandler, view: ViewConfig) -> Self {
        Self {
            view: Rc::new(Mutex::new(view)),
            // editor: Rc::new(editor),
            files: Rc::new(Mutex::new(file_handler)),
        }
    }

}
