use super::configurations::external_editor::ExternalEditor;
use super::configurations::global_variables_files::GlobalVariablesFiles;
use super::configurations::view::ViewConfig;
use super::configurations::Configuration;
use crate::base::os::file_edition_handler::FileEditionHandler;
use crate::config::configurations::save_files::SaveFiles;
use crate::utils::file_facades::{data_file::DataFile, request_file::RequestFile, FileFacade};
use std::rc::Rc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct ConfigManager {
    pub saved_requests: Rc<Mutex<SaveFiles>>,
    pub editor: Rc<ExternalEditor>,
    pub view: Rc<Mutex<ViewConfig>>,
    pub edition_files_handler: Rc<Mutex<FileEditionHandler>>,
    pub global_variables: Rc<Mutex<GlobalVariablesFiles>>,
}
impl ConfigManager {
    pub fn init() -> Self {
        let saved_requests = SaveFiles::setup_and_init().unwrap();
        let editor = ExternalEditor::setup_and_init().unwrap();
        let global_variables = GlobalVariablesFiles::setup_and_init().unwrap();
        let view = ViewConfig::init();
        Self {
            saved_requests: Rc::new(Mutex::new(saved_requests)),
            view: Rc::new(Mutex::new(view)),
            editor: Rc::new(editor),
            edition_files_handler: Rc::new(Mutex::new(FileEditionHandler::default())),
            global_variables: Rc::new(Mutex::new(global_variables)),
        }
    }

    pub fn setup_env() -> Result<(), String> {
        RequestFile::setup()?;
        DataFile::setup()?;
        Ok(())
    }
}
