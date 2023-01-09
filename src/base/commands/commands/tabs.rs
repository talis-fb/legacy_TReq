use crate::base::web::request::Request;
use crate::commands::{Command, Commands};
use crate::states::{self, State};
use crate::App;

impl Commands {
    pub fn go_to_next_tab() -> Command {
        |app: &mut App| {
            app.get_data_store_mut().goto_next_request();
            Ok(())
        }
    }

    pub fn go_to_previous_tab() -> Command {
        |app: &mut App| {
            app.get_data_store_mut().goto_prev_request();
            Ok(())
        }
    }

    pub fn add_new_tab() -> Command {
        |app: &mut App| {
            let req = Request::default();
            app.get_data_store_mut().add_request();
            Ok(())
        }
    }

    pub fn rename_tab() -> Command {
        |app: &mut App| {
            app.set_input_mode_with_command(
                |app: &mut App| {
                    let buffer = app.get_input_buffer_value();
                    let data_store = app.get_data_store_mut();

                    let mut req = (*data_store.get_request()).clone();
                    req.set_name(buffer);

                    data_store.update_request(req.clone());
                    Ok(())
                },
                app.get_data_store().get_request().name.clone(),
            );
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

    pub fn go_to_tab(i: usize) -> Command {
        |app: &mut App| {
            // app.get_data_store_mut().goto_request(i);
            Ok(())
        }
    }
}
