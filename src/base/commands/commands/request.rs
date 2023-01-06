use crate::base::web::request::METHODS;
use crate::commands::{Command, Commands};
use crate::states::{self, State, States};
use crate::App;
use std::process::{Stdio, Command as OSCommand};

impl Commands {
    pub fn switch_request_options() -> Command {
        |app: &mut App| Ok(())
    }
    pub fn edit_request_body_vim() -> Command {
        |app: &mut App| {
            app.set_vim_mode_with_command(
                |app: &mut App| {
                    let buffer = app.get_input_buffer();
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
                    let buffer = app.get_input_buffer();
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
    pub fn edit_request_headers() -> Command {
        |app: &mut App| {
            app.set_new_state(States::TabActiveState::init());
            Ok(())
        }
    }
    pub fn switch_request_method() -> Command {
        |app: &mut App| {
            let method_stack = [
                METHODS::GET,
                METHODS::DELETE,
                METHODS::HEAD,
                METHODS::PATCH,
                METHODS::POST,
                METHODS::PUT,
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
                    let buffer = app.get_input_buffer();
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
