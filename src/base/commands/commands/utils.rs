use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn do_nothing() -> Command {
        |app: &mut App| Ok(())
    }

    pub fn err() -> Command {
        |app: &mut App| Err("Ai".to_string())
    }
}
