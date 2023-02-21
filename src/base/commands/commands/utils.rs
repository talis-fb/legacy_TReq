use std::rc::Rc;

use crate::app::InputMode;
use crate::base::commands::CommandTrait;
use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn do_nothing() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }

    pub fn err() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                Err("Ai".to_string())
            }
        }

        Rc::new(Box::new(S {}))
    }

    pub fn show_help() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.clear_log();
                app.set_mode(InputMode::Help);
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }

    pub fn quit() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.is_finished = true;
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }
}
