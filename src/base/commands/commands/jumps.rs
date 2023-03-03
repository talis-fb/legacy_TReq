use crate::base::commands::CommandTrait;
use crate::base::states::states::{self, State};
use crate::commands::{Command, Commands};
use crate::App;
use std::sync::Arc;

impl Commands {
    pub fn go_to_tab_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::TabActiveState::init());
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
    pub fn go_to_url_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::RequestUrlActiveState::init());
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
    pub fn go_to_request_body_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::RequestActiveState::init());
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
    pub fn go_to_request_header_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::RequestHeaderActiveState::init());
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
    pub fn go_to_response_body_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::ResponseBodyActiveState::init());
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
    pub fn go_to_response_headers_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::ResponseHeadersState::init());
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
    pub fn go_to_log_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::LogsState::init());
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
}
