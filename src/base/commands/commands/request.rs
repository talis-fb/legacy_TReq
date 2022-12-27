use crate::commands::{Command, Commands};
use crate::states::{self, State, States};
use crate::App;

impl Commands {
    pub fn switch_request_options() -> Command {
        |app: &mut App| Ok(())
    }
    pub fn edit_request_body() -> Command {
        |app: &mut App| {
            app.set_input_mode_with_callback(|app: &mut App, s: String| {
                let data_store = app.get_data_store_mut();
                let mut req = (*data_store.get_request()).clone();
                req.set_url(s);
                data_store.update_request(req);
            });
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
            // app.current_state = Box::new(states::active_request_body::/**/));
            Ok(())
        }
    }
    pub fn edit_request_url() -> Command {
        |app: &mut App| {
            app.set_input_mode_with_callback(|app: &mut App, s: String| {
                let data_store = app.get_data_store_mut();
                let mut req = (*data_store.get_request()).clone();
                req.set_url(s);
                data_store.update_request(req);
            });
            Ok(())
        }
    }
}
