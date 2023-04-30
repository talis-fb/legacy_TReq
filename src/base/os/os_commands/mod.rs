use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::Arc;

pub mod external_editor;

#[mockall::automock]
pub trait OsCommandTrait<Arg: 'static, Output: 'static> {
    // Block current thread until return response of command
    fn sync_open(&self, args: Arg) -> Result<Output, String>;

    // Spawn a thread to run process asynchronous
    fn async_open(&self, args: Arg) -> Result<mpsc::Receiver<Output>, String>;

    fn is_valid(&self) -> bool;

    fn init() -> Result<Self, String>
    where
        Self: Sized;
}

pub enum OsCommand {
    Sync(Arc<Box<dyn OsCommandTrait<PathBuf, String> + Send + Sync>>),
    Async(Arc<Box<dyn OsCommandTrait<PathBuf, String> + Send + Sync>>),
}

impl OsCommand {
    fn create_sync_from<T>(value: T) -> Self
    where
        T: OsCommandTrait<PathBuf, String> + Send + Sync,
    {
        OsCommand::Sync(Arc::new(Box::new(value)))
    }

    fn create_async_from<T>(value: T) -> Self
    where
        T: OsCommandTrait<PathBuf, String> + Send + Sync,
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
