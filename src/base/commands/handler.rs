use crate::app::app::App;

use super::Command;

pub struct CommandHandler;
impl CommandHandler {
    pub fn execute(app: &mut App, command: Command) -> Result<(), String> {
        command(app)
    }
}
