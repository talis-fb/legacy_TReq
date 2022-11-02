use crate::commands::{CommandFunc, CommandsList};
use crate::request::Request;
use crate::states::{self, State};
use crate::App;

impl CommandsList {
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

    pub fn rename_tab() -> CommandFunc {
        |app: &mut App| {
            let req = Request::default();
            app.create_request(req);
            Ok(())
        }
    }

    pub fn delete_tab() -> CommandFunc {
        |app: &mut App| {
            let req = Request::default();
            app.create_request(req);
            Ok(())
        }
    }

    pub fn go_to_tab(n: usize) -> CommandFunc {
        |app: &mut App| {
            let req = Request::default();
            app.create_request(req);
            Ok(())
        }
    }
}
