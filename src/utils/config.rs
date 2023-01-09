use std::{
    collections::HashMap,
    fmt::Display,
    fs::{File, OpenOptions},
    io::Write,
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
    rc::Rc, sync::RwLock,
};

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

#[derive(Clone)]
pub struct AppFile {
    path: PathBuf,
}
impl AppFile {
    pub fn init(path: PathBuf) -> Self {
        Self {
            path,
            // file_open: None,
        }
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
        file.write_all(content.as_bytes()).unwrap();
        Ok(())
    }

    // pub fn create(path: String, extension: Option<String>) -> Result<Self, String> {
    //     let path = Path::new(APP_DATA_PATH);
    //     println!("path: {}", path.display());
    //     let mut path = path.join(path);
    //
    //     println!("path: {}", path.display());
    //     path.set_extension(extension.as_ref().unwrap_or(&String::new()));
    //
    //     println!("path: {}", path.display());
    //
    //     let file = File::create(&path).map_err(|e| e.to_string())?;
    //     let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    //     Ok(Self {
    //         file,
    //         content,
    //         extension: None,
    //     })
    // }
}

// pub struct TomlFile {
//     pub file: AppFile,
// }
// impl TomlFile {
//     pub fn create(name: String) -> Result<Self, String> {
//         let file = AppFile::create(name, Some(String::from("toml")))?;
//         Ok(Self { file })
//     }
//
//     pub fn update_content(&mut self, content: String) -> Result<(), String> {
//         self.file.update_content(content)
//     }
// }
//
// pub struct JsonFile {
//     pub file: AppFile,
// }
// impl JsonFile {
//     pub fn create(name: String) -> Result<Self, String> {
//         let file = AppFile::create(name, Some(String::from("json")))?;
//         Ok(Self { file })
//     }
//
//     pub fn update_content(&mut self, content: String) -> Result<(), String> {
//         self.file.update_content(content)
//     }
// }
