use std::{hash::Hash, path::PathBuf};

use serde::{Deserialize, Serialize};

use super::{external_editor::ExternalEditor, OsCommandTrait};
use crate::{base::{commands::Command, os::file_facades::FileFacade}, utils::custom_types::uuid::UUID};

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
