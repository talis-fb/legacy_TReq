use crate::app::App;
use crate::app::InputMode;
use crate::base::commands::CommandTrait;
use crate::base::commands::{Command, Commands};

impl Commands {
    pub fn do_nothing() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, _app: &mut App) -> Result<(), String> {
                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn err() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, _app: &mut App) -> Result<(), String> {
                Err("Ai".to_string())
            }
        }

        Commands::from(S {})
    }

    pub fn undo_state() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.reset_to_last_state();
                Ok(())
            }
        }

        Commands::from(S {})
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

        Commands::from(S {})
    }

    pub fn quit() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.is_finished = true;
                Ok(())
            }
        }

        Commands::from(S {})
    }
}
