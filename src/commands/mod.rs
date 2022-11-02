use crate::{app::App, request::Request};
use std::collections::HashMap;

type CommandFunc = fn(app: &mut App) -> Result<(), String>;

pub struct CommandsList {}
impl CommandsList {
    pub fn do_nothing() -> CommandFunc {
        |app: &mut App| Ok(())
    }

    pub fn go_to_next_tab() -> CommandFunc {
        |app: &mut App| {
            if app.current_request >= app.get_requests().len() - 1 {
                app.current_request = 0;
                return Ok(());
            }
            app.current_request += 1;
            Ok(())
        }
    }

    pub fn err() -> CommandFunc {
        |app: &mut App| Err("Ai".to_string())
    }

    pub fn go_to_previous_tab() -> CommandFunc {
        |app: &mut App| {
            if app.current_request == 0 {
                app.current_request = app.get_requests().len() - 1;
                return Ok(());
            }
            app.current_request -= 1;
            Ok(())
        }
    }
    pub fn add_new_tab() -> CommandFunc {
        |app: &mut App| {
            let req = Request::default();
            app.create_request(req);
            Ok(())
        }
    }
}
