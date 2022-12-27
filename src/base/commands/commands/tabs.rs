use crate::base::web::request::Request;
use crate::commands::{Command, Commands};
use crate::states::{self, State};
use crate::App;

impl Commands {
    pub fn go_to_next_tab() -> Command {
        |app: &mut App| {
            // if app.current_request >= app.get_requests().len() - 1 {
            //     app.current_request = 0;
            //     app.get_data_store().set_request_to_index(0);
            //     return Ok(());
            // }
            //
            // app.current_request += 1;
            Ok(())
        }
    }

    pub fn go_to_previous_tab() -> Command {
        |app: &mut App| {
            // if app.current_request == 0 {
            //     app.current_request = app.get_requests().len() - 1;
            //     return Ok(());
            // }
            // app.current_request -= 1;
            Ok(())
        }
    }

    pub fn add_new_tab() -> Command {
        |app: &mut App| {
            let req = Request::default();
            // app.create_request(req);
            Ok(())
        }
    }

    pub fn rename_tab() -> Command {
        |app: &mut App| {
            let req = Request::default();
            // app.create_request(req);
            Ok(())
        }
    }

    pub fn delete_tab() -> Command {
        |app: &mut App| {
            let req = Request::default();
            // app.create_request(req);
            Ok(())
        }
    }

    pub fn go_to_tab(n: usize) -> Command {
        |app: &mut App| {
            let req = Request::default();
            // app.create_request(req);
            Ok(())
        }
    }
}
