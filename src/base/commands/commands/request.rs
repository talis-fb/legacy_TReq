use crate::app::App;
use crate::base::commands::CommandTrait;
use crate::base::commands::{Command, Commands};

use crate::base::web::request::METHODS;
use crate::utils::custom_types::uuid::UUID;

use std::collections::HashMap;

impl Commands {
    pub fn save_request() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let result = app.get_data_store_mut().save_request();

                match result {
                    Err(e) => app
                        .get_data_store_mut()
                        .set_log_error("ERROR SAVE REQUEST".to_string(), e),
                    Ok(_) => app
                        .get_data_store_mut()
                        .set_log_helping("SAVED".to_string(), "".to_string()),
                }

                Ok(())
            }
        }

        Commands::from(S {})
    }
    pub fn switch_request_options() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, _app: &mut App) -> Result<(), String> {
                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn switch_request_method() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let method_stack = [
                    METHODS::GET,
                    METHODS::POST,
                    METHODS::PUT,
                    METHODS::PATCH,
                    METHODS::DELETE,
                    METHODS::HEAD,
                ];
                let mut new_req = (*app.get_data_store().get_request()).clone();

                let current = method_stack
                    .into_iter()
                    .position(|i| i == new_req.method)
                    .unwrap_or(0);
                let next = (current + 1) % method_stack.len();

                new_req.method = method_stack[next];
                app.get_data_store_mut().update_request(new_req);
                Ok(())
            }
        }

        Commands::from(S {})
    }
    pub fn edit_request_url() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                struct _S;
                impl CommandTrait for _S {
                    fn execute(&self, app: &mut App) -> Result<(), String> {
                        let buffer = app.get_input_buffer_value();
                        let data_store = app.get_data_store_mut();

                        let mut req = (*data_store.get_request()).clone();
                        req.set_url(buffer);

                        data_store.update_request(req.clone());
                        Ok(())
                    }
                }

                app.set_input_mode_with_command(
                    Commands::from(_S {}),
                    app.get_data_store().get_request().url.clone(),
                );
                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn set_input_buffer_to_request_body() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let buffer = app.get_input_buffer_value();
                let data_store = app.get_data_store_mut();

                let mut req = (*data_store.get_request()).clone();
                req.set_body(buffer);

                data_store.update_request(req);
                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn set_input_buffer_to_request_headers() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let buffer = app.get_input_buffer();
                let header_map: HashMap<String, String> = serde_json::from_str(&buffer.value)
                    .unwrap_or_else(|e| {
                        let store = app.get_data_store_mut();
                        store.set_log_error(String::from("ERROR HEADERS"), e.to_string());

                        // If there is some value to header before the failed editing, rollback
                        // to it, otherwise get some empty object
                        store.input_buffer.reset_to_backup();
                        let buffer_backup_str = &store.input_buffer.value;
                        let buffer_backup_map = serde_json::from_str(buffer_backup_str);
                        buffer_backup_map.unwrap_or_default()
                    });

                let data_store = app.get_data_store_mut();
                let mut req = (*data_store.get_request()).clone();
                req.set_headers(header_map);

                data_store.update_request(req);
                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn edit_request_body_vim() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let uuid = format!(
                    "TREQ-req-body-{}",
                    app.get_data_store().get_request_uuid().value
                );

                let command = Commands::open_editor_to_buffer(
                    Commands::set_input_buffer_to_request_body(),
                    Some(UUID::from(uuid)),
                    Some(app.get_data_store().get_request().body.clone()),
                );

                command.execute(app)?;

                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn edit_request_headers_vim() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let initial_headers = app.get_data_store().get_request().headers.clone();
                let initial_headers_as_str =
                    serde_json::to_string_pretty(&initial_headers).unwrap_or_default();

                let uuid = format!(
                    "TREQ-req-headers-{}",
                    app.get_data_store().get_request_uuid().value
                );

                let command = Commands::open_editor_to_buffer(
                    Commands::set_input_buffer_to_request_headers(),
                    Some(UUID::from(uuid)),
                    Some(initial_headers_as_str),
                );

                command.execute(app)?;

                Ok(())
            }
        }

        Commands::from(S {})
    }
    pub fn restart_body_of_file() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let uuid = UUID::from(format!(
                    "TREQ-req-body-{}",
                    app.get_data_store().get_request_uuid().value
                ));

                let store = app.get_data_store();
                let file_handler_cc = store.config.files.clone();
                let mut file_handler = file_handler_cc.try_lock().map_err(|e| e.to_string())?;

                let buffer = file_handler.get_content_temp_file(&uuid)?;

                app.set_input_buffer_value(buffer);
                Commands::set_input_buffer_to_request_body().execute(app)?;

                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn restart_headers_of_file() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let uuid = UUID::from(format!(
                    "TREQ-req-headers-{}",
                    app.get_data_store().get_request_uuid().value
                ));

                let data_store = app.get_data_store_mut();
                let file_handler_cc = data_store.config.files.clone();
                let mut file_handler = file_handler_cc.try_lock().map_err(|e| e.to_string())?;

                let buffer = file_handler.get_content_temp_file(&uuid)?;

                app.set_input_buffer_value(buffer);
                Commands::set_input_buffer_to_request_headers().execute(app)?;

                Ok(())
            }
        }

        Commands::from(S {})
    }
}
