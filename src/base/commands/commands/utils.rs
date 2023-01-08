use crate::app::app::InputMode;
use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn do_nothing() -> Command {
        |app: &mut App| Ok(())
    }

    pub fn err() -> Command {
        |app: &mut App| Err("Ai".to_string())
    }

    pub fn show_help() -> Command {
        |app: &mut App| {
            app.set_mode(InputMode::Help);
            Ok(())
        }
    }

    pub fn quit() -> Command {
        |app: &mut App| {
            app.is_finished = true;
            Ok(())
        }
    }
}
