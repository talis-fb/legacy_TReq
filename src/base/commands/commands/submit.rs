use crate::commands::{Command, Commands};
use crate::states::{self, State, States};
use crate::App;

impl Commands {
    pub fn submit() -> Command {
        |app: &mut App| {
            app.set_new_state(States::TabActiveState::init());
            Ok(())
        }
    }
}
