use crate::app::App;

pub type Command = fn(app: &mut App) -> Result<(), String>;

mod commands;
pub struct Commands;

pub mod handler;
