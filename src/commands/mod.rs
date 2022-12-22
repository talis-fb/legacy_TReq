use crate::{
    app::App,
    request::Request,
    states::{self, State},
};
use std::collections::HashMap;

pub type Command = fn(app: &mut App) -> Result<(), String>;

mod jumps;
mod logs;
mod request;
mod response;
mod tabs;

pub struct Commands {}
impl Commands {
    pub fn do_nothing() -> Command {
        |app: &mut App| Ok(())
    }

    pub fn err() -> Command {
        |app: &mut App| Err("Ai".to_string())
    }
}
