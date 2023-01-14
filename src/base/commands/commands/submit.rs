use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn submit() -> Command {
        |app: &mut App| {
            app.dispatch_submit();
            Ok(())
        }
    }
}
