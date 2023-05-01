use super::file_facades::requests::RequestFile;
use super::file_facades::temp_edition::TempEditionfile;
use super::file_facades::variables::VariablesFile;
use super::file_facades::FileFacade;
use crate::base::web::request::Request;
use crate::utils::custom_types::uuid::UUID;
use std::collections::HashMap;
use std::path::PathBuf;

type BoxRequestFile = Box<dyn FileFacade<UUID, Request>>;
type BoxTempEditionfile = Box<dyn FileFacade<UUID, String>>;
type BoxVariablesFile = Box<dyn FileFacade<String, HashMap<String, String>>>;

pub trait FileFactory {
    fn create_request_file(&mut self, id: UUID, request: Request) -> Result<BoxRequestFile, String>;

    fn create_variables_file(
        &mut self,
        id: String,
        variables: HashMap<String, String>,
    ) -> Result<BoxVariablesFile, String>;

    fn create_temp_file(&mut self, id: UUID, content: String) -> Result<BoxTempEditionfile, String>;

    fn get_saved_request_file(&self, path: PathBuf) -> Result<BoxRequestFile, String>;
    fn get_saved_variables_file(&self, path: PathBuf) -> Result<BoxVariablesFile, String>;
    fn get_saved_temp_file(&self, path: PathBuf) -> Result<BoxTempEditionfile, String>;

    // Utils
    fn get_saved_files_request(&self) -> Result<Vec<BoxRequestFile>, String>;
}

#[derive(Default)]
pub struct FileDefaultFactory;

impl FileFactory for FileDefaultFactory {
    fn create_temp_file(&mut self, id: UUID, content: String) -> Result<BoxTempEditionfile, String> {
        let file = TempEditionfile::create(id, content)?;
        Ok(Box::new(file))
    }

    fn create_request_file(&mut self, id: UUID, request: Request) -> Result<BoxRequestFile, String> {
        let file = RequestFile::create(id, request)?;
        Ok(Box::new(file))
    }

    fn create_variables_file(
        &mut self,
        id: String,
        variables: HashMap<String, String>,
    ) -> Result<BoxVariablesFile, String> {
        let file = VariablesFile::create(id, variables)?;
        Ok(Box::new(file))
    }

    fn get_saved_request_file(&self, path: PathBuf) -> Result<BoxRequestFile, String> {
        let file = RequestFile { path };
        file.get_content()?;
        Ok(Box::new(file))
    }

    fn get_saved_variables_file(&self, path: PathBuf) -> Result<BoxVariablesFile, String> {
        let file = VariablesFile { path };
        file.get_content()?;
        Ok(Box::new(file))
    }

    fn get_saved_temp_file(&self, path: PathBuf) -> Result<BoxTempEditionfile, String> {
        let file = TempEditionfile { path };
        file.get_content()?;
        Ok(Box::new(file))
    }

    fn get_saved_files_request(&self) -> Result<Vec<BoxRequestFile>, String> {
        let mut all_files: Vec<Box<dyn FileFacade<UUID, Request>>> = vec![];
        let root_path = RequestFile::get_root_path();
        let paths = std::fs::read_dir(root_path).map_err(|e| e.to_string())?;
        for entry in paths {
            let path = entry.map_err(|e| e.to_string())?.path();
            let request_file = self.get_saved_request_file(path)?;

            // Verify if content in File is valid
            if request_file.get_content().is_ok() {
                all_files.push(request_file);
            }
        }

        Ok(all_files)
    }
}
