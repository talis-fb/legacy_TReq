use std::sync::Arc;
use std::rc::Rc;

use crate::base::commands::CommandTrait;
use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn submit() -> Command {
        struct S;
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                app.dispatch_submit();
                Ok(())
            }
        }

        Arc::new(Box::new(S {}))
    }
}
