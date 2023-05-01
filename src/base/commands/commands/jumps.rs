use crate::app::App;
use crate::base::commands::CommandTrait;
use crate::base::commands::{Command, Commands};
use crate::base::states::states::{self, State};

impl Commands {
    pub fn go_to_tab_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::TabActiveState::init());
                Ok(())
            }
        }

        Commands::from(S {})
    }
    pub fn go_to_url_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::RequestUrlActiveState::init());
                Ok(())
            }
        }

        Commands::from(S {})
    }
    pub fn go_to_request_body_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::RequestActiveState::init());
                app.get_data_store_mut().view.request.open_body_view();
                Ok(())
            }
        }

        Commands::from(S {})
    }
    pub fn go_to_request_header_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::RequestHeaderActiveState::init());
                app.get_data_store_mut().view.request.open_headers_view();
                Ok(())
            }
        }

        Commands::from(S {})
    }
    pub fn go_to_response_body_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::ResponseBodyActiveState::init());
                app.get_data_store_mut().view.response.open_body_view();
                Ok(())
            }
        }

        Commands::from(S {})
    }
    pub fn go_to_response_headers_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::ResponseHeadersState::init());
                app.get_data_store_mut().view.response.open_headers_view();
                Ok(())
            }
        }

        Commands::from(S {})
    }
    pub fn go_to_log_section() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::LogsState::init());
                Ok(())
            }
        }

        Commands::from(S {})
    }
}
