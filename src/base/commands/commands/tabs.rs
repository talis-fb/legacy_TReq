use std::sync::Arc;

use crate::base::commands::CommandTrait;
use crate::base::web::request::Request;
use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn go_to_next_tab() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.get_data_store_mut().goto_next_request();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn go_to_previous_tab() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.get_data_store_mut().goto_prev_request();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn add_new_tab() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let req = Request::default();
                app.get_data_store_mut().add_request();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn rename_tab() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                struct _S;
                impl CommandTrait for _S {
                    fn execute(&self, app: &mut App) -> Result<(), String> {
                        let buffer = app.get_input_buffer_value();
                        let data_store = app.get_data_store_mut();

                        let mut req = (*data_store.get_request()).clone();
                        req.set_name(buffer);

                        data_store.update_request(req.clone());
                        Ok(())
                    }
                }

                app.set_input_mode_with_command(
                    Arc::new(Box::new(_S {})),
                    app.get_data_store().get_request().name.clone(),
                );
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn delete_tab() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.get_data_store_mut().delete_current_request();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn go_to_tab(i: usize) -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
}
