use crate::commands::{CommandFunc, CommandsList};
use crate::states::{self, State};
use crate::App;

impl CommandsList {
    pub fn go_to_tab_section() -> CommandFunc {
        |app: &mut App| {
            app.current_state = Box::new(states::active_tablist::TabActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_url_section() -> CommandFunc {
        |app: &mut App| {
            app.current_state = Box::new(states::active_request_url::TabActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_request_body_section() -> CommandFunc {
        |app: &mut App| {
            app.current_state = Box::new(states::active_request_body::TabActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_response_body_section() -> CommandFunc {
        |app: &mut App| {
            app.current_state =
                Box::new(states::active_response_body::RequestBodyActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_log_section() -> CommandFunc {
        |app: &mut App| {
            app.current_state = Box::new(states::active_logs::TabActiveState::init());
            Ok(())
        }
    }
}
