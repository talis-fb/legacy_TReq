use std::collections::HashMap;
use std::path::PathBuf;

use super::file_facade::MockFile;
use treq::utils::custom_types::uuid::UUID;
use treq::base::web::request::Request;
use treq::base::os::file_factory::FileFactory;
use treq::base::os::file_facades::FileFacade;

type BoxRequestFile = Box<dyn FileFacade<UUID, Request>>;
type BoxTempEditionfile = Box<dyn FileFacade<UUID, String>>;
type BoxVariablesFile = Box<dyn FileFacade<String, HashMap<String, String>>>;

pub struct MockFileFactory;

impl FileFactory for MockFileFactory {

    fn create_request_file(&self, id: UUID, request: Request) -> Result<BoxRequestFile, String> {
        Ok(Box::new(MockFile::create(id, request)?))
    }


    fn create_variables_file(
        &self,
        id: String,
        variables: HashMap<String, String>,
    ) -> Result<BoxVariablesFile, String> {
        Ok(Box::new(MockFile::create(id, variables)?))
    }

    fn create_temp_file(&self, id: UUID, content: String) -> Result<BoxTempEditionfile, String> {
        Ok(Box::new(MockFile::create(id, content)?))
    }

    fn get_saved_request_file(&self, path: PathBuf) -> Result<BoxRequestFile, String> {
        todo!()
    }
    fn get_saved_variables_file(&self, path: PathBuf) -> Result<BoxVariablesFile, String> {
        todo!()
    }
    fn get_saved_temp_file(&self, path: PathBuf) -> Result<BoxTempEditionfile, String> {
        todo!()
    }

    // Utils
    fn get_saved_files_request(&self) -> Result<Vec<BoxRequestFile>, String> {
        todo!()
    }
}
