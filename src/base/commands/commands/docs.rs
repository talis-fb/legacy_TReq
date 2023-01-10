use crate::app::app::InputMode;
use crate::base::doc::DocsFactory;
use crate::commands::{Command, Commands};
use crate::states::{self, State, States};
use crate::App;

impl Commands {
    pub fn open_help_screen() -> Command {
        |app: &mut App| {
            app.get_data_store_mut().doc_reader = Some(DocsFactory::help_reader());
            app.set_mode(InputMode::Help);
            Ok(())
        }
    }
}
