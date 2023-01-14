use crate::config::configurations::save_files::SaveFiles;
use crate::utils::file_facades::{data_file::DataFile, request_file::RequestFile, FileFacade};
use super::configurations::external_editor::ExternalEditor;

pub struct ConfigManager {
    pub saved_requests: SaveFiles,
    pub editor: ExternalEditor
}
impl ConfigManager {
    pub fn setup_env() -> Result<(), String> {
        RequestFile::setup()?;
        DataFile::setup()?;
        Ok(())
    }
}
