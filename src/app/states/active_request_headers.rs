use crate::actions::Actions;
use crate::states::{Commands, CommandsMap, State, StatesNames};
use std::collections::HashMap;

pub struct RequestHeaderActiveState {
    pub maps: CommandsMap,
}
impl State for RequestHeaderActiveState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::RequestHeaders
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (Actions::Edit, Commands::edit_request_body()),
                (Actions::Switch, Commands::switch_request_options()),
                (Actions::Up, Commands::go_to_url_section()),
                (Actions::Down, Commands::go_to_log_section()),
            ]),
        }
    }
}
