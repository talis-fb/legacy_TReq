use crate::config::configurations::save_files::SaveFiles;
use crate::utils::file_facades::{data_file::DataFile, request_file::RequestFile, FileFacade};

pub struct ConfigManager {
    pub saved_requests: SaveFiles,
}
impl ConfigManager {
    pub fn setup_env() -> Result<(), String> {
        RequestFile::setup()?;
        DataFile::setup()?;
        Ok(())
    }
}
