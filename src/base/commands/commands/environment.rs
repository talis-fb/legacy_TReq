use crate::base::commands::CommandTrait;
use crate::commands::{Command, Commands};
use crate::view::views::environment::store::OpenedVars;
use crate::App;
use std::sync::Arc;

impl Commands {
    pub fn go_to_next_global_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();

                let max_index = store.environment.global.len() - 1;
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

                let new_index: usize = (store.view.environment.current_session_var - 1)
                    .try_into()
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

                let max_index = store.environment.session.len() - 1;
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

                let new_index: usize = (store.view.environment.current_global_var - 1)
                    .try_into()
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
                let current = store.view.environment.opened_section;

                let command_to_exec = match current {
                    OpenedVars::Session => Commands::open_global_env_vars(),
                    OpenedVars::Global => Commands::open_session_env_vars(),
                };

                command_to_exec.execute(app);

                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
}
