use crate::app::InputMode;
use crate::base::commands::CommandTrait;

use crate::app::App;
use crate::base::commands::{Command, Commands};

impl Commands {
    // TODO: Move it to another place
    pub fn set_input_buffer(content: String) -> Command {
        struct S {
            content: String,
        }

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_input_buffer_value(self.content.clone());
                Ok(())
            }
        }

        Commands::from(S { content })
    }

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

        Commands::from(S { char: c })
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

        Commands::from(S {})
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

        Commands::from(S {})
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

        Commands::from(S {})
    }

    pub fn edit_mode_delete_all() -> Command {
        struct S;

        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let input_buffer = app.get_input_buffer_mut();
                input_buffer.go_to_start();
                input_buffer.delete_till_end();
                Ok(())
            }
        }

        Commands::from(S {})
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

        Commands::from(S {})
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

        Commands::from(S {})
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

        Commands::from(S {})
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

        Commands::from(S {})
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

        Commands::from(S {})
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

        Commands::from(S {})
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

        Commands::from(S {})
    }
}
