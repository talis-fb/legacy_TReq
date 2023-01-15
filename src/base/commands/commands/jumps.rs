use crate::commands::{Command, Commands};
use crate::states::{State, self};
use crate::App;

impl Commands {
    pub fn go_to_tab_section() -> Command {
        |app: &mut App| {
            app.set_new_state(states::TabActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_url_section() -> Command {
        |app: &mut App| {
            app.set_new_state(states::RequestUrlActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_request_body_section() -> Command {
        |app: &mut App| {
            app.set_new_state(states::RequestActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_request_header_section() -> Command {
        |app: &mut App| {
            app.set_new_state(states::RequestHeaderActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_response_body_section() -> Command {
        |app: &mut App| {
            app.set_new_state(states::ResponseBodyActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_response_headers_section() -> Command {
        |app: &mut App| {
            app.set_new_state(states::ResponseHeadersState::init());
            Ok(())
        }
    }
    pub fn go_to_log_section() -> Command {
        |app: &mut App| {
            app.set_new_state(states::LogsState::init());
            Ok(())
        }
    }
}
