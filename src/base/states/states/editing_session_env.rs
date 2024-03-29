use crate::base::actions::Actions;
use crate::base::commands::Commands;
use crate::base::states::states::{CommandsMap, State, StatesNames};
use std::collections::HashMap;

pub struct EditingSessionEnvState {
    pub maps: CommandsMap,
}
impl State for EditingSessionEnvState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::EditingSessionEnv
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (Actions::Quit, Commands::exit_environment_view()),
                (Actions::Submit, Commands::exit_environment_view()),
                (Actions::Switch, Commands::switch_opened_env_vars()),
                (Actions::Edit, Commands::edit_current_session_env_var()),
                (Actions::Up, Commands::go_to_prev_session_env_var()),
                (Actions::Down, Commands::go_to_next_session_env_var()),
                (Actions::New, Commands::add_session_env_var()),
                (Actions::Delete, Commands::remove_current_session_env_var()),
            ]),
        }
    }
}
