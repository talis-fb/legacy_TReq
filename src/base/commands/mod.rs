use std::sync::Arc;

use crate::app::App;

pub trait CommandTrait {
    fn execute(&self, app: &mut App) -> Result<(), String>;
}

// It needs to be a Box, a Struct which implements 'CommandTrait'
// but, it need to be cloned, to do so... it needs to be a Rc
pub type Command = Arc<Box<dyn CommandTrait + Send + Sync>>;

mod commands;
pub struct Commands;

pub mod handler;
