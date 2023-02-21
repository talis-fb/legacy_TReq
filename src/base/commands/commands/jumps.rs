use crate::base::commands::CommandTrait;
use crate::base::states::states::{self, State};
use crate::commands::{Command, Commands};
use crate::App;
use std::rc::Rc;

impl Commands {
    pub fn go_to_tab_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::TabActiveState::init());
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }
    pub fn go_to_url_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::RequestUrlActiveState::init());
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }
    pub fn go_to_request_body_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::RequestActiveState::init());
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }
    pub fn go_to_request_header_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::RequestHeaderActiveState::init());
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }
    pub fn go_to_response_body_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::ResponseBodyActiveState::init());
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }
    pub fn go_to_response_headers_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::ResponseHeadersState::init());
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }
    pub fn go_to_log_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::LogsState::init());
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }
}
