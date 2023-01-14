use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::prelude::OsStrExt;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::RwLock;

use directories::ProjectDirs;

pub struct FileUtils;
impl FileUtils {
    pub fn create_path_if_it_does_not_exist(path: &PathBuf) -> Result<(), String> {
        if !std::fs::metadata(path).is_ok() {
            std::fs::create_dir_all(path).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub fn get_data_dir() -> Option<PathBuf> {
        let project = ProjectDirs::from("", "", "TReq")?;
        Some(project.data_dir().to_path_buf())
    }

    pub fn get_config_dir() -> Option<PathBuf> {
        let project = ProjectDirs::from("", "", "TReq")?;
        Some(project.config_dir().to_path_buf())
    }
}

pub trait IFile {
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
}


#[derive(Clone, Debug)]
pub struct DataFile {
    path: PathBuf,
}
impl IFile for DataFile {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    fn get_parent_path() -> PathBuf {
        FileUtils::get_data_dir().unwrap().join("data")
    }

    fn from_name(filename: String) -> Self {
        let path = Self::get_parent_path().join(filename);
        Self { path }
    }
}


#[derive(Clone, Debug)]
pub struct RequestFile {
    pub path: PathBuf,
}
impl RequestFile {
    pub fn from_path(path: PathBuf) -> Self {
        Self { path }
    }
}
impl IFile for RequestFile {
    fn get_path(&self) -> PathBuf {
        self.path.clone()
    }
    fn get_parent_path() -> PathBuf {
        FileUtils::get_data_dir().unwrap().join("requests")
    }

    fn from_name(filename: String) -> Self {
        let path = Self::get_parent_path().join(filename);
        Self { path }
    }
}
