use std::rc::Rc;

use crate::base::commands::CommandTrait;
use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn edit_response_vim() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let response = app.get_data_store().get_response().lock().unwrap().clone();

                // It opens Editor with response Data, and does nothing when finished

                struct _S;
                impl CommandTrait for _S {
                    fn execute(&self, app: &mut App) -> Result<(), String> {
                        Ok(())
                    }
                }

                app.set_vim_mode_with_command(Rc::new(Box::new(_S {})), response.body);

                Ok(())
            }
        }

        Rc::new(Box::new(S {}))
    }
}
