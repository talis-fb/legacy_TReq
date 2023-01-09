use crate::base::logs::{Log, LogType};
use crate::base::web::request::METHODS;
use crate::commands::{Command, Commands};
use crate::states::{self, State, States};
use crate::App;
use std::collections::HashMap;
use std::process::{Command as OSCommand, Stdio};
use std::time::Duration;

impl Commands {
    pub fn save_request() -> Command {
        |app: &mut App| {
            let result = app.get_data_store_mut().save_request();

            match result {
                Err(e) => app.get_data_store_mut().set_log_error("ERROR SAVE REQUEST".to_string(), e.to_string()),
                Ok(_) => app.get_data_store_mut().set_log_helping("SAVED".to_string(), "".to_string()),
            }
            
            Ok(())
        }
    }
    pub fn switch_request_options() -> Command {
        |app: &mut App| Ok(())
    }
    pub fn edit_request_body_vim() -> Command {
        |app: &mut App| {
            app.set_vim_mode_with_command(
                |app: &mut App| {
                    let buffer = app.get_input_buffer_value();
                    let data_store = app.get_data_store_mut();

                    let mut req = (*data_store.get_request()).clone();
                    req.set_body(buffer);

                    data_store.update_request(req);
                    Ok(())
                },
                app.get_data_store().get_request().body.clone(),
            );
            Ok(())
        }
    }
    pub fn edit_request_body() -> Command {
        |app: &mut App| {
            app.set_input_mode_with_command(
                |app: &mut App| {
                    let buffer = app.get_input_buffer_value();
                    let data_store = app.get_data_store_mut();

                    let mut req = (*data_store.get_request()).clone();
                    req.set_body(buffer);

                    data_store.update_request(req);
                    Ok(())
                },
                app.get_data_store().get_request().body.clone(),
            );
            Ok(())
        }
    }
    pub fn edit_request_headers_vim() -> Command {
        |app: &mut App| {
            let initial_headers = app.get_data_store().get_request().headers.clone();
            let initial_headers_as_str =
                serde_json::to_string_pretty(&initial_headers).unwrap_or(String::new());

            app.set_vim_mode_with_command(
                |app: &mut App| {
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
                            buffer_backup_map.unwrap_or(HashMap::new())
                        });

                    let data_store = app.get_data_store_mut();
                    let mut req = (*data_store.get_request()).clone();
                    req.set_headers(header_map);

                    data_store.update_request(req);
                    Ok(())
                },
                initial_headers_as_str,
            );
            Ok(())
        }
    }
    pub fn switch_request_method() -> Command {
        |app: &mut App| {
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
    pub fn edit_request_url() -> Command {
        |app: &mut App| {
            app.set_input_mode_with_command(
                |app: &mut App| {
                    let buffer = app.get_input_buffer_value();
                    let data_store = app.get_data_store_mut();

                    let mut req = (*data_store.get_request()).clone();
                    req.set_url(buffer);

                    data_store.update_request(req.clone());
                    Ok(())
                },
                app.get_data_store().get_request().url.clone(),
            );
            Ok(())
        }
    }
}
