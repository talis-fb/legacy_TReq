use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use std::sync::Arc;

use crate::base::commands::Command;

pub mod external_editor;

#[mockall::automock]
pub trait OsCommandTrait {
    fn exec(&self, sender: Sender<Command>) -> Result<(), String>;
}

// #[mockall::automock]
// pub trait OsCommandTrait<Output: 'static> {
//     // Block current thread until return response of command
//     fn sync_open(&self) -> Result<Output, String>;
//
//     // Spawn a thread to run process asynchronous
//     fn async_open(&self) -> Result<mpsc::Receiver<Output>, String>;
//
//     fn is_valid(&self) -> bool;
//
//     // fn init() -> Result<Self, String>
//     // where
//     //     Self: Sized;
// }

#[derive(Clone)]
pub enum OsCommand {
    Sync(Arc<Box<dyn OsCommandTrait + Send + Sync>>),
    Async(Arc<Box<dyn OsCommandTrait + Send + Sync>>),
}

impl OsCommand {
    fn create_sync_from<T>(value: T) -> Self
    where
        T: OsCommandTrait + 'static + Send + Sync,
    {
        OsCommand::Sync(Arc::new(Box::new(value)))
    }

    fn create_async_from<T>(value: T) -> Self
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
