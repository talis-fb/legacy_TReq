pub mod data_file;
pub mod request_file;
pub mod edition_file;

use super::files::FileUtils;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

pub trait FileFacade {
    fn get_path(&self) -> PathBuf;
    fn from_name(filename: String) -> Self;
    fn get_parent_path() -> PathBuf;

    /// Setup the parent folders of the Struct
    fn setup() -> Result<(), String> {
        let path = Self::get_parent_path();
        FileUtils::create_path_if_it_does_not_exist(&path)
    }

    /// Return a instance of File opened
    fn open_or_create_file(&self) -> Result<File, String> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(self.get_path())
            .map_err(|e| e.to_string())?;
        Ok(file)
    }

    fn get_content(&self) -> Result<String, String> {
        std::fs::read_to_string(self.get_path()).map_err(|e| e.to_string())
    }

    fn save_content(&mut self, content: String) -> Result<(), String> {
        let mut file = self.open_or_create_file()?;
        file.set_len(0).map_err(|e| e.to_string())?;
        file.write_all(content.as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn remove(&mut self) -> Result<(), String> {
        std::fs::remove_file(self.get_path()).map_err(|e| e.to_string())
    }
}
