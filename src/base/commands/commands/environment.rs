use crate::base::commands::CommandTrait;
use crate::base::states::states::{self, State};
use crate::commands::{Command, Commands};
use crate::view::views::environment::store::OpenedVars;
use crate::App;
use std::sync::Arc;

impl Commands {
    pub fn open_environment_view() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let opened = app.get_data_store_mut().view.environment.opened_section;

                let command_to_exec = match opened {
                    OpenedVars::Session => Commands::open_global_env_vars(),
                    OpenedVars::Global => Commands::open_session_env_vars(),
                };

                command_to_exec.execute(app)
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn exit_environment_view() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::DefaultState::init());
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn go_to_next_global_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();

                let max_index = store.environment.global.len().checked_sub(1).unwrap_or(0);
                let current_index = store.view.environment.current_global_var;

                store.view.environment.current_global_var =
                    std::cmp::min(current_index + 1, max_index);

                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn go_to_prev_global_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();

                let new_index: usize = store
                    .view
                    .environment
                    .current_session_var
                    .checked_sub(1)
                    .unwrap_or(0);

                store.view.environment.current_global_var = new_index;

                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn edit_current_global_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn go_to_next_session_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();

                let max_index = store.environment.session.len().checked_sub(1).unwrap_or(0);
                let current_index = store.view.environment.current_session_var;

                store.view.environment.current_session_var =
                    std::cmp::min(current_index + 1, max_index);

                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn go_to_prev_session_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();

                let new_index: usize = store
                    .view
                    .environment
                    .current_session_var
                    .checked_sub(1)
                    .unwrap_or(0);

                store.view.environment.current_session_var = new_index;

                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn edit_current_session_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn open_global_env_vars() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::EditingGlobalEnvState::init());
                app.get_data_store_mut().view.environment.opened_section = OpenedVars::Global;
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn open_session_env_vars() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::EditingSessionEnvState::init());
                app.get_data_store_mut().view.environment.opened_section = OpenedVars::Session;
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }

    pub fn switch_opened_env_vars() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();
                let current = &store.view.environment.opened_section;

                let command_to_exec = match current {
                    OpenedVars::Session => Commands::open_global_env_vars(),
                    OpenedVars::Global => Commands::open_session_env_vars(),
                };

                command_to_exec.execute(app)
            }
        }

        Arc::new(Box::new(S {}))
    }
}
