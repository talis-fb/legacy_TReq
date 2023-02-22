use super::Actions;
use crate::{base::commands::Command, base::states::manager::StateManager};

#[derive(Clone)]
pub struct ActionsManager {
    last_command: Option<Command>
}
impl ActionsManager {
    pub fn init() -> Self {
        Self { last_command: None }
    }

    pub fn get_command_of_action(&mut self, action: Actions, states: &StateManager) -> Option<Command> {
        let commands_map = states.get_command_map();
        let command = commands_map.get(&action)?;
        self.last_command = Some(command.clone());
        Some(command.clone())
    }
}
