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
                (Actions::Edit, Commands::edit_request_headers_vim()),
                (Actions::Switch, Commands::go_to_request_body_section()),
                (Actions::Up, Commands::go_to_url_section()),
                (Actions::Down, Commands::go_to_log_section()),
            ]),
        }
    }
}
