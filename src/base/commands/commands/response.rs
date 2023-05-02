use crate::app::App;
use crate::base::commands::CommandTrait;
use crate::base::commands::{Command, Commands};

impl Commands {
    pub fn edit_response_vim() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let command = Commands::open_editor_to_buffer(
                    Commands::do_nothing(),
                    app.get_data_store().get_request().body.clone(),
                    None,
                );

                command.execute(app)?;

                Ok(())
            }
        }

        Commands::from(S {})
    }
}
