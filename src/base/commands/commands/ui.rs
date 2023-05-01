use crate::app::App;
use crate::base::commands::CommandTrait;
use crate::base::commands::{Command, Commands};

impl Commands {
    pub fn grow_right_ui() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.get_data_store_mut()
                    .config
                    .view
                    .lock()
                    .unwrap()
                    .grow_right_block();
                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn grow_left_ui() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.get_data_store_mut()
                    .config
                    .view
                    .lock()
                    .unwrap()
                    .grow_left_block();
                Ok(())
            }
        }

        Commands::from(S {})
    }
}
