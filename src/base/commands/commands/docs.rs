use std::sync::Arc;

use crate::app::InputMode;
use crate::base::commands::CommandTrait;
use crate::base::doc::DocsFactory;
use crate::base::commands::{Command, Commands};
use crate::app::App;

impl Commands {
    pub fn open_help_screen() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.get_data_store_mut().doc_reader = Some(DocsFactory::help_reader());
                app.set_mode(InputMode::Help);
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn open_welcome_screen() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.get_data_store_mut().doc_reader = Some(DocsFactory::welcome_reader());
                app.set_mode(InputMode::Help);
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn doc_up() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let doc_reader = app.get_data_store_mut().doc_reader.as_mut();

                if doc_reader.is_none() {
                    return Err("There is not doc to read".to_string());
                }

                doc_reader.unwrap().go_to_prev_row();

                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn doc_down() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let doc_reader = app.get_data_store_mut().doc_reader.as_mut();

                if doc_reader.is_none() {
                    return Err("There is not doc to read".to_string());
                }

                doc_reader.unwrap().go_to_next_row();

                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn doc_exit() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_mode(InputMode::Normal);
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
}
