use std::sync::Arc;

use crate::app::InputMode;
use crate::base::commands::CommandTrait;

use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn type_char_edit_mode(c: char) -> Command {
        struct S {
            char: char,
        }

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let result = app.get_data_store_mut().save_request();

                let mut buffer = app.get_input_buffer_value();
                buffer.push(self.char);

                app.set_input_buffer_value(buffer);

                Ok(())
            }
        }

        Arc::new(Box::new(S { char: c }))
    }

    pub fn erase_last_char_edit_mode() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let result = app.get_data_store_mut().save_request();

                let mut buffer = app.get_input_buffer_value();
                buffer.pop();

                app.set_input_buffer_value(buffer);

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
