use crate::commands::{Command, Commands};
use crate::states::{self, State, States};
use crate::App;

impl Commands {
    pub fn go_to_tab_section() -> Command {
        |app: &mut App| {
            app.set_new_state(States::TabActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_url_section() -> Command {
        |app: &mut App| {
            app.set_new_state(States::RequestUrlActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_request_body_section() -> Command {
        |app: &mut App| {
            app.set_new_state(States::RequestActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_request_header_section() -> Command {
        |app: &mut App| {
            app.set_new_state(States::RequestHeaderActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_response_body_section() -> Command {
        |app: &mut App| {
            app.set_new_state(States::ResponseBodyActiveState::init());
            Ok(())
        }
    }
    pub fn go_to_response_headers_section() -> Command {
        |app: &mut App| {
            app.set_new_state(States::ResponseHeadersState::init());
            Ok(())
        }
    }
    pub fn go_to_log_section() -> Command {
        |app: &mut App| {
            app.set_new_state(States::LogsState::init());
            Ok(())
        }
    }
}
