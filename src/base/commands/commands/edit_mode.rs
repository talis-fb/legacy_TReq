use std::sync::Arc;

use crate::app::InputMode;
use crate::base::commands::CommandTrait;

use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn edit_mode_insert_char(c: char) -> Command {
        struct S {
            char: char,
        }

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let input_buffer = app.get_input_buffer_mut();
                input_buffer.insert_char(self.char);
                Ok(())
            }
        }

        Arc::new(Box::new(S { char: c }))
    }

    pub fn edit_mode_delete_prev_char() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let input_buffer = app.get_input_buffer_mut();
                input_buffer.delete_prev_char();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn edit_mode_delete_next_char() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let input_buffer = app.get_input_buffer_mut();
                input_buffer.delete_next_char();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn edit_mode_delete_till_end() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let input_buffer = app.get_input_buffer_mut();
                input_buffer.delete_till_end();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn edit_mode_go_to_prev_char() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let input_buffer = app.get_input_buffer_mut();
                input_buffer.go_to_prev_char();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn edit_mode_go_to_next_char() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let input_buffer = app.get_input_buffer_mut();
                input_buffer.go_to_next_char();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn edit_mode_go_to_start() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let input_buffer = app.get_input_buffer_mut();
                input_buffer.go_to_start();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn edit_mode_go_to_end() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let input_buffer = app.get_input_buffer_mut();
                input_buffer.go_to_end();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn process_edit_mode() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.exec_input_buffer_command()?;
                Commands::close_edit_mode().execute(app)?;
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn cancel_edit_mode() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.get_input_buffer_mut().reset_to_backup();

                Commands::close_edit_mode().execute(app)?;

                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn close_edit_mode() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.clear_log();
                app.set_mode(InputMode::Normal);
                app.reset_to_last_state();

                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
}
