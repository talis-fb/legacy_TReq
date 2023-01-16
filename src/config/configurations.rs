pub mod external_editor;
pub mod save_files;

use std::collections::HashMap;

pub trait Configuration<K, File, EntityFile> {
    fn get_map(&self) -> &HashMap<K, File>;
    fn get_as_file(&self, key: &K) -> Option<&File>;
    fn get_as_entity(&self, key: &K) -> Result<EntityFile, String>;
    fn setup_and_init() -> Result<Self, String>
    where
        Self: Sized;
}

pub trait ConfigurationEditable<K, File, EntityFile>: Configuration<K, File, EntityFile> {
    fn set(&mut self, key: &K, value: &EntityFile) -> Result<(), String>;
}
