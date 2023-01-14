use crate::utils::files::{RequestFile, DataFile, IFile};
use self::saves::SaveFiles;

pub mod saves;
pub mod setup;
pub mod trat;



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

