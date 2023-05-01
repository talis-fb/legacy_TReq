use std::path::PathBuf;

use super::{external_editor::ExternalEditor, OsCommandTrait};
use crate::base::commands::Command;

#[mockall::automock]
pub trait OsCommandFactory {
    fn external_editor(
        &self,
        // path: &Box<dyn FileFacade<UUID, String>>,
        path: PathBuf,
        command: Command,
    ) -> Result<Box<dyn OsCommandTrait + Send + Sync>, String>;
    // where
    //     File: FileFacade<A, B> + 'static,
    //     A: PartialEq + Eq + Hash + 'static,
    //     B: for<'a> Deserialize<'a> + Serialize + Clone + 'static;
}

pub struct OsCommandDefaultFactory;
impl OsCommandFactory for OsCommandDefaultFactory {
    fn external_editor(
        &self,
        path: PathBuf,
        command: Command,
    ) -> Result<Box<dyn OsCommandTrait + Send + Sync>, String> {
        Ok(Box::new(ExternalEditor::init(path, command)?))
    }
}
