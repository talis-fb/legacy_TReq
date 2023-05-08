use crate::app::App;
use crate::base::commands::CommandTrait;
use crate::base::commands::{Command, Commands};

impl Commands {
    pub fn edit_response_body_vim() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let response = {
                    app.get_data_store()
                        .get_response()
                        .lock()
                        .unwrap()
                        .body
                        .clone()
                };

                let command =
                    Commands::open_editor_to_buffer(Commands::do_nothing(), None, Some(response));

                command.execute(app)?;

                Ok(())
            }
        }

        Commands::from(S {})
    }

    pub fn edit_response_headers_vim() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let initial_headers_as_str = {
                    let headers = app
                        .get_data_store()
                        .get_response()
                        .lock()
                        .unwrap()
                        .headers
                        .clone();
                    serde_json::to_string_pretty(&headers).unwrap_or_default()
                };

                let command = Commands::open_editor_to_buffer(
                    Commands::do_nothing(),
                    None,
                    Some(initial_headers_as_str),
                );

                command.execute(app)?;

                Ok(())
            }
        }

        Commands::from(S {})
    }
}
