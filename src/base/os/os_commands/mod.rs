use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use crate::base::commands::Command;

pub mod external_editor;
pub mod factory;

#[mockall::automock]
pub trait OsCommandTrait {
    fn exec(&self, sender: Sender<Command>) -> Result<(), String>;
}

#[derive(Clone)]
pub enum OsCommand {
    Sync(Arc<Box<dyn OsCommandTrait + Send + Sync>>),
    Async(Arc<Box<dyn OsCommandTrait + Send + Sync>>),
}

impl OsCommand {
    pub fn create_sync_from<T>(value: T) -> Self
    where
        T: OsCommandTrait + 'static + Send + Sync,
    {
        OsCommand::Sync(Arc::new(Box::new(value)))
    }

    pub fn create_async_from<T>(value: T) -> Self
    where
        T: OsCommandTrait + 'static + Send + Sync,
    {
        OsCommand::Async(Arc::new(Box::new(value)))
    }
}
