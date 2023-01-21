use crate::utils::custom_types::uuid::UUID;
use crate::utils::file_facades::edition_file::EditionFile;
use crate::utils::file_facades::FileFacade;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Default)]
pub struct FileEditionHandler {
    files: HashMap<UUID, EditionFile>,
}
impl FileEditionHandler {
    fn create_or_open(&mut self, uuid: &UUID) -> &mut EditionFile {
        self.files
            .entry(uuid.clone())
            .or_insert(EditionFile::from_name(uuid.value.clone()))
    }

    pub fn get_content(&mut self, uuid: &UUID) -> Result<String, String> {
        let file = self.create_or_open(uuid);
        file.get_content()
    }

    pub fn save_content(&mut self, uuid: &UUID, content: String) -> Result<(), String> {
        let file = self.create_or_open(uuid);
        file.save_content(content)
    }

    pub fn get_path(&mut self, uuid: &UUID) -> PathBuf {
        let file = self.create_or_open(uuid);
        file.get_path()
    }
}
impl Drop for FileEditionHandler {
    fn drop(&mut self) {
        self.files
            .values_mut()
            .into_iter()
            .for_each(|f| f.remove().unwrap());
    }
}
