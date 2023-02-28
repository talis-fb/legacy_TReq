use std::sync::{Arc, Mutex};

use crate::app::App;

#[derive(Clone, Copy)]
pub enum CommandType {
    Sync,
    Async,
}

pub trait CommandTrait {
    fn execute(&self, app: &mut App) -> Result<(), String>;

    fn type_running(&self) -> CommandType {
        CommandType::Sync
    }

    // --- ONLY TO ASYNC Commands ---
    fn get_id(&self) -> String {
        String::new()
    }
    fn take_task(&self) -> Option<tokio::task::JoinHandle<Command>> {
        None
    }
    fn is_task_begin(&self) -> bool {
        false
    }
}

// It needs to be a Box, a Struct which implements 'CommandTrait'
// but, it need to be cloned, to do so... it needs to be a Rc
pub type Command = Arc<Box<dyn CommandTrait + Send + Sync>>;

mod commands;
pub struct Commands;

pub mod handler;
