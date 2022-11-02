use crate::commands::{CommandFunc, CommandsList};
use crate::states::{self, State};
use crate::App;

impl CommandsList {
    pub fn switch_request_options() -> CommandFunc {
        |app: &mut App| Ok(())
    }
    pub fn edit_request_body() -> CommandFunc {
        |app: &mut App| {
            app.set_input_mode_with_callback(|app: &mut App, s: String| {
                let mut req = app.get_current_request().clone();
                req.set_url(s.clone());
                app.set_current_request(req);
            });
            Ok(())
        }
    }
    pub fn edit_request_headers() -> CommandFunc {
        |app: &mut App| {
            app.current_state = Box::new(states::active_tablist::TabActiveState::init());
            Ok(())
        }
    }
    pub fn switch_request_method() -> CommandFunc {
        |app: &mut App| {
            // app.current_state = Box::new(states::active_request_body::/**/));
            Ok(())
        }
    }
    pub fn edit_request_url() -> CommandFunc {
        |app: &mut App| {
            app.set_input_mode_with_callback(|app: &mut App, s: String| {
                let mut req = app.get_current_request().clone();
                req.set_url(s.clone());
                app.set_current_request(req);
            });
            Ok(())
        }
    }
}
