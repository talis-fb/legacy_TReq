use crate::app::App;

use super::Command;

#[derive(Clone)]
pub struct CommandHandler;
impl CommandHandler {
    pub fn execute(app: &mut App, command: Command) -> Result<(), String> {
        command.execute(app)
    }
}
