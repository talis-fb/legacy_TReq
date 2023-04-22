use super::file_facades::requests::RequestFile;
use super::file_facades::temp_edition::TempEditionfile;
use super::file_facades::variables::VariablesFile;
use super::file_facades::FileFacade;
use crate::base::web::request::Request;
use crate::utils::custom_types::uuid::UUID;
use std::collections::HashMap;

type BoxRequestFile = Box<dyn FileFacade<UUID, Request>>;
type BoxTempEditionfile = Box<dyn FileFacade<UUID, String>>;
type BoxVariablesFile = Box<dyn FileFacade<String, HashMap<String, String>>>;

pub trait FileFactory {
    fn create_request_file(id: UUID, request: Request) -> Result<BoxRequestFile, String>
    where
        Self: Sized;

    fn create_variables_file(
        id: String,
        variables: HashMap<String, String>,
    ) -> Result<BoxVariablesFile, String>
    where
        Self: Sized;

    fn create_temp_file(id: UUID, content: String) -> Result<BoxTempEditionfile, String>
    where
        Self: Sized;
}

struct FileDefaultFactory;
impl FileFactory for FileDefaultFactory {
    fn create_temp_file(id: UUID, content: String) -> Result<BoxTempEditionfile, String> {
        let file = TempEditionfile::create(id, content)?;
        Ok(Box::new(file))
    }

    fn create_request_file(id: UUID, request: Request) -> Result<BoxRequestFile, String> {
        let file = RequestFile::create(id, request)?;
        Ok(Box::new(file))
    }

    fn create_variables_file(
        id: String,
        variables: HashMap<String, String>,
    ) -> Result<BoxVariablesFile, String> {
        let file = VariablesFile::create(id, variables)?;
        Ok(Box::new(file))
    }
}