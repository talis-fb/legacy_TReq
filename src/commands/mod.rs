use crate::{
    app::App,
    request::Request,
    states::{self, State},
};
use std::collections::HashMap;

pub type CommandFunc = fn(app: &mut App) -> Result<(), String>;

mod jumps;
mod tabs;

pub struct CommandsList {}
impl CommandsList {
    pub fn do_nothing() -> CommandFunc {
        |app: &mut App| Ok(())
    }

    pub fn err() -> CommandFunc {
        |app: &mut App| Err("Ai".to_string())
    }
}
