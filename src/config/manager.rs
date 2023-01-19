use std::rc::Rc;
use std::sync::Mutex;
use super::configurations::Configuration;
use super::configurations::external_editor::ExternalEditor;
use super::configurations::view::ViewConfig;
use crate::config::configurations::save_files::SaveFiles;
use crate::utils::file_facades::{data_file::DataFile, request_file::RequestFile, FileFacade};

#[derive(Clone)]
pub struct ConfigManager {
    pub saved_requests: Rc<Mutex<SaveFiles>>,
    pub editor: Rc<ExternalEditor>,
    pub view: Rc< Mutex<ViewConfig>>,
}
impl ConfigManager {
    pub fn init() -> Self {
        let saved_requests = SaveFiles::setup_and_init().unwrap();
        let editor = ExternalEditor::setup_and_init().unwrap();
        let view = ViewConfig::init();
        Self {
            saved_requests: Rc::new(Mutex::new(saved_requests)),
            view: Rc::new(Mutex::new(view)),
            editor: Rc::new(editor),
        }
    }

    pub fn setup_env() -> Result<(), String> {
        RequestFile::setup()?;
        DataFile::setup()?;
        Ok(())
    }
}
