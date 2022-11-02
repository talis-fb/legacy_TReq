use crate::commands::{CommandFunc, CommandsList};
use crate::states::{self, State};
use crate::App;

impl CommandsList {
    pub fn switch_request_options() -> CommandFunc {
        |app: &mut App| Ok(())
    }
    pub fn edit_request_body() -> CommandFunc {
        |app: &mut App| {
            app.current_state = Box::new(states::active_tablist::TabActiveState::init());
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
            app.current_state = Box::new(states::active_tablist::TabActiveState::init());
            Ok(())
        }
    }
}
