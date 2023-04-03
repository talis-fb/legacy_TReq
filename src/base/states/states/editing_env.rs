use crate::actions::Actions;
use crate::base::states::states::{CommandsMap, State, StatesNames};
use crate::commands::Commands;
use std::collections::HashMap;

pub struct EditingEnvState {
    pub maps: CommandsMap,
}
impl State for EditingEnvState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::EditingEnv
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                // (Actions::Edit, Commands::edit_request_body_vim()),
                (Actions::Switch, Commands::switch_opened_env_vars()),
                // (Actions::Up, Commands::go_to_url_section()),
                // (Actions::Right, Commands::go_to_response_body_section()),
            ]),
        }
    }
}
