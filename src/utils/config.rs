use std::collections::HashMap;
use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::os::unix::prelude::OsStrExt;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::RwLock;

pub static APP_DATA_PATH: &str = "/home/talis/.local/share/treq/";
pub static APP_DATA_PATH_REQUESTS: &str = "/home/talis/.local/share/treq/requests";

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct UUID {
    pub value: String,
}
impl UUID {
    pub fn new() -> Self {
        UUID {
            value: uuid::Uuid::new_v4().to_string(),
        }
    }
    pub fn from(value: String) -> Self {
        UUID { value }
    }
}

#[derive(Clone, Debug)]
pub struct AppFile {
    path: PathBuf,
}
impl AppFile {
    pub fn init(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn open_or_create_file(&mut self) -> Result<File, String> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&self.path)
            .map_err(|e| e.to_string())?;
        Ok(file)
    }

    pub fn get_content(&self) -> Result<String, String> {
        std::fs::read_to_string(&self.path).map_err(|e| e.to_string())
    }

    pub fn save_content(&mut self, content: String) -> Result<(), String> {
        let mut file = self.open_or_create_file()?;
        file.set_len(0).map_err(|e| e.to_string())?;
        file.write_all(content.as_bytes())
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
