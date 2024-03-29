use crate::app::App;
use crate::base::commands::CommandTrait;
use crate::base::commands::{Command, Commands};
use crate::base::states::states::{self, State};
use crate::view::views::environment::store::OpenedVars;

impl Commands {
    pub fn open_environment_view() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let _store = app.get_data_store_mut();

                // TODO:
                // Probaly this will fail, once it clone keys to View Store in
                // ever opened of Pop up. This should be smarter

                let store = app.get_data_store_mut();
                let g = &store.environment.global;
                let s = &store.environment.session;
                store.view.environment.sync(g, s);

                let opened = store.view.environment.opened_section;

                let command_to_exec = match opened {
                    OpenedVars::Session => Commands::open_session_env_vars(),
                    OpenedVars::Global => Commands::open_global_env_vars(),
                };

                command_to_exec.execute(app)
            }
        }

        Commands::from(S {})
    }

    pub fn exit_environment_view() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.set_new_state(states::DefaultState::init());
                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn go_to_next_global_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();

                let max_index = store.environment.global.len().saturating_sub(1);
                let current_index = store.view.environment.current_global_var;

                store.view.environment.current_global_var =
                    std::cmp::min(current_index + 1, max_index);

                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn go_to_prev_global_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();

                let new_index: usize = store.view.environment.current_session_var.saturating_sub(1);

                store.view.environment.current_global_var = new_index;

                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn go_to_next_session_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();

                let max_index = store.environment.session.len().saturating_sub(1);
                let current_index = store.view.environment.current_session_var;

                store.view.environment.current_session_var =
                    std::cmp::min(current_index + 1, max_index);

                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn go_to_prev_session_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();

                let new_index: usize = store.view.environment.current_session_var.saturating_sub(1);

                store.view.environment.current_session_var = new_index;

                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn edit_current_global_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                struct _S;
                impl CommandTrait for _S {
                    fn execute(&self, app: &mut App) -> Result<(), String> {
                        let new_value = app.get_input_buffer_value();

                        let store = app.get_data_store_mut();
                        let var_key_active = store
                            .view
                            .environment
                            .get_current_var_key()
                            .ok_or("no var key".to_string())?;
                        let value = store.environment.global.get_mut(&var_key_active).unwrap();
                        *value = new_value;

                        store.environment.save_globals()?;

                        Ok(())
                    }
                }

                let store = app.get_data_store_mut();
                let var_key_active = store
                    .view
                    .environment
                    .get_current_var_key()
                    .ok_or("no var key".to_string())?;
                let value = store
                    .environment
                    .global
                    .get(&var_key_active)
                    .cloned()
                    .unwrap();

                app.set_input_mode_with_command(Commands::from(_S {}), value);
                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn edit_current_session_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                struct _S;
                impl CommandTrait for _S {
                    fn execute(&self, app: &mut App) -> Result<(), String> {
                        let new_value = app.get_input_buffer_value();

                        let store = app.get_data_store_mut();
                        let var_key_active = store
                            .view
                            .environment
                            .get_current_var_key()
                            .ok_or("no var key".to_string())?;
                        let value = store.environment.session.get_mut(&var_key_active).unwrap();
                        *value = new_value;

                        Ok(())
                    }
                }

                let store = app.get_data_store_mut();
                let var_key_active = store
                    .view
                    .environment
                    .get_current_var_key()
                    .ok_or("no var key".to_string())?;
                let value = store
                    .environment
                    .session
                    .get(&var_key_active)
                    .cloned()
                    .unwrap();

                app.set_input_mode_with_command(Commands::from(_S {}), value);
                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn add_global_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                struct _S;
                impl CommandTrait for _S {
                    fn execute(&self, app: &mut App) -> Result<(), String> {
                        let new_key_value = app.get_input_buffer_value();
                        let _value = app
                            .get_data_store_mut()
                            .environment
                            .global
                            .insert(new_key_value, String::new());

                        let store = app.get_data_store_mut();
                        let g = &store.environment.global;
                        let s = &store.environment.session;
                        store.view.environment.sync(g, s);

                        store.environment.save_globals()?;

                        Ok(())
                    }
                }

                app.set_input_mode_with_command(Commands::from(_S {}), "new_title".to_string());
                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn add_session_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                struct _S;
                impl CommandTrait for _S {
                    fn execute(&self, app: &mut App) -> Result<(), String> {
                        let new_key_value = app.get_input_buffer_value();
                        let _value = app
                            .get_data_store_mut()
                            .environment
                            .session
                            .insert(new_key_value, String::new());

                        let store = app.get_data_store_mut();
                        let g = &store.environment.global;
                        let s = &store.environment.session;
                        store.view.environment.sync(g, s);

                        Ok(())
                    }
                }

                app.set_input_mode_with_command(Commands::from(_S {}), "new_title".to_string());
                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn remove_current_global_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();
                let current_key = store
                    .view
                    .environment
                    .get_current_var_key()
                    .ok_or("no var key".to_string())?;
                store.environment.global.remove(&current_key);

                let store = app.get_data_store_mut();
                let g = &store.environment.global;
                let s = &store.environment.session;
                store.view.environment.sync(g, s);

                store.environment.save_globals()?;

                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn remove_current_session_env_var() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let store = app.get_data_store_mut();
                let current_key = store
                    .view
                    .environment
                    .get_current_var_key()
                    .ok_or("no var key".to_string())?;

                store.environment.session.remove(&current_key);

                let store = app.get_data_store_mut();
                let g = &store.environment.global;
                let s = &store.environment.session;
                store.view.environment.sync(g, s);

                Ok(())
            }
        }

        Commands::from(S {})
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

        Commands::from(S {})
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

        Commands::from(S {})
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

        Commands::from(S {})
    }
}
