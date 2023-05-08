use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use super::file_facade::MockFile;
use treq::base::os::file_facades::FileFacade;
use treq::base::os::file_factory::FileFactory;
use treq::base::web::request::Request;
use treq::utils::custom_types::uuid::UUID;

type BoxRequestFile = Box<dyn FileFacade<UUID, Request>>;
type BoxTempEditionfile = Box<dyn FileFacade<UUID, String>>;
type BoxVariablesFile = Box<dyn FileFacade<String, HashMap<String, String>>>;

#[derive(Default)]
pub struct MockFileFactory {
    requests: HashSet<UUID>,
    variables: HashSet<String>,
    temp: HashSet<UUID>,
}

impl FileFactory for MockFileFactory {
    fn create_request_file(
        &mut self,
        id: UUID,
        request: Request,
    ) -> Result<BoxRequestFile, String> {
        let does_not_contains_before = self.requests.insert(id.clone());

        if !does_not_contains_before {
            panic!("Creating a file already created");
        }

        Ok(Box::new(MockFile::create(id, request)?))
    }

    fn create_variables_file(
        &mut self,
        id: String,
        variables: HashMap<String, String>,
    ) -> Result<BoxVariablesFile, String> {
        let does_not_contains_before = self.variables.insert(id.clone());

        if !does_not_contains_before {
            panic!("Creating a file already created");
        }

        Ok(Box::new(MockFile::create(id, variables)?))
    }

    fn create_temp_file(
        &mut self,
        id: UUID,
        content: String,
    ) -> Result<BoxTempEditionfile, String> {
        let does_not_contains_before = self.temp.insert(id.clone());

        if !does_not_contains_before {
            panic!("Creating a file already created");
        }

        Ok(Box::new(MockFile::create(id, content)?))
    }

    fn get_saved_request_file(&self, _path: PathBuf) -> Result<BoxRequestFile, String> {
        todo!()
    }
    fn get_saved_variables_file(&self, _path: PathBuf) -> Result<BoxVariablesFile, String> {
        todo!()
    }
    fn get_saved_temp_file(&self, _path: PathBuf) -> Result<BoxTempEditionfile, String> {
        todo!()
    }

    // Utils
    fn get_saved_files_request(&self) -> Result<Vec<BoxRequestFile>, String> {
        todo!()
    }
}
