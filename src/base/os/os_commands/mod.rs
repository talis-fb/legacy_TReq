use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use std::sync::Arc;

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
    // Sync(Arc<Box<dyn OsCommandTrait + Send + Sync>>),
    // Async(Arc<Box<dyn OsCommandTrait + Send + Sync>>),
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

// pub struct Opa;
// impl OsCommandTrait<i32, i64> for Opa {
//     fn is_valid(&self) -> bool {
//         todo!()
//     }
//     fn sync_open(&self,args:i32) -> Result<i64,String> {
//         todo!()
//     }
//     fn async_open(&self,args:i32) -> Result<mpsc::Receiver<i64>,String> {
//         todo!()
//     }
//     fn init() -> Result<Self,String>where Self:Sized {
//         todo!()
//     }
// }
