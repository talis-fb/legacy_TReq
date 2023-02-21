use std::rc::Rc;

use crate::app::InputMode;
use crate::base::commands::CommandTrait;
use crate::base::doc::DocsFactory;
use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn open_help_screen() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.get_data_store_mut().doc_reader = Some(DocsFactory::help_reader());
                app.set_mode(InputMode::Help);
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }

    pub fn open_welcome_screen() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.get_data_store_mut().doc_reader = Some(DocsFactory::welcome_reader());
                app.set_mode(InputMode::Help);
                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }
}
