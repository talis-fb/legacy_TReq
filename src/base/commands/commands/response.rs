use std::sync::Arc;

use crate::base::commands::CommandTrait;
use crate::base::commands::{Command, Commands};
use crate::app::App;

impl Commands {
    pub fn edit_response_vim() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let response = app.get_data_store().get_response().lock().unwrap().clone();

                // It opens Editor with response Data, and does nothing when finished
                app.set_vim_mode_with_command(Commands::do_nothing(), response.body);

                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
}
