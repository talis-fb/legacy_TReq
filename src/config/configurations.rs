pub mod external_editor;
pub mod global_variables_files;
pub mod save_files;
pub mod view;

use crate::utils::file_facades::FileFacade;
use std::{collections::HashMap, hash::Hash};

pub trait Configuration<FileID, FileBuf, FileEntity>
where
    FileID: PartialEq + Eq + Hash,
    FileBuf: FileFacade,
{
    fn get_map(&self) -> &HashMap<FileID, FileBuf>;
    fn get_as_file(&self, key: &FileID) -> Option<&FileBuf>;
    fn get_as_entity(&self, key: &FileID) -> Result<FileEntity, String>;
    fn setup_and_init() -> Result<Self, String>
    where
        Self: Sized;
}

pub trait ConfigurationEditable<FileID, FileBuf, FileEntity>:
    Configuration<FileID, FileBuf, FileEntity>
where
    FileID: PartialEq + Eq + Hash,
    FileBuf: FileFacade,
{
    fn set(&mut self, key: &FileID, value: &FileEntity) -> Result<(), String>;
}
