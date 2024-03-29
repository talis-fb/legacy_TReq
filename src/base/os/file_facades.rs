use serde::{self, Deserialize, Serialize};

use crate::utils::files::FileUtils;
use std::fs::OpenOptions;
use std::hash::Hash;
use std::io::Write;
use std::path::PathBuf;

// #[cfg(test)]
// use mockall::{automock, mock, predicate::*};

pub mod requests;
pub mod temp_edition;
pub mod variables;

#[cfg_attr(test, mockall::automock)]
pub trait FileFacade<FileID: 'static, FileEntity: 'static>
where
    FileID: PartialEq + Eq + Hash,
    FileEntity: for<'a> Deserialize<'a> + Serialize + Clone,
{
    /// Setup the parent folders of the Struct
    fn setup() -> Result<(), String>
    where
        Self: Sized,
    {
        let path = Self::get_root_path();
        FileUtils::create_path_if_it_does_not_exist(&path)
    }

    fn get_content(&self) -> Result<FileEntity, String> {
        let content = std::fs::read_to_string(self.get_path()).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    fn save_content(&mut self, value: FileEntity) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(self.get_path())
            .map_err(|e| e.to_string())?;

        file.set_len(0).map_err(|e| e.to_string())?;

        let content: String = serde_json::to_string_pretty(&value).map_err(|e| e.to_string())?;

        file.write_all(content.as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn remove(&mut self) -> Result<(), String> {
        std::fs::remove_file(self.get_path()).map_err(|e| e.to_string())
    }

    // Must defines
    fn get_root_path() -> PathBuf
    where
        Self: Sized;
    fn get_path(&self) -> PathBuf;
    fn create(id: FileID, value: FileEntity) -> Result<Self, String>
    where
        Self: Sized;
}
