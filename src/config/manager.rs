use crate::config::configurations::save_files::SaveFiles;
use crate::utils::file_facades::{FileFacade, request_file::RequestFile, data_file::DataFile};

pub struct ConfigManager {
    pub saved_requests: SaveFiles
}
impl ConfigManager {
    pub fn setup_env() -> Result<(), String> {
        RequestFile::setup()?;
        DataFile::setup()?;
        Ok(())
    }
}

