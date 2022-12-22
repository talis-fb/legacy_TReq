use crate::events::Actions;
use crate::states::{CommandFunc, Commands, Map, State, StatesNames};
use std::collections::HashMap;

pub struct TabActiveState {
    pub maps: Map,
}
impl State for TabActiveState {
    fn get_map(&self) -> &Map {
        &self.maps
    }
    fn get_state_name(&self) -> StatesNames {
        StatesNames::RequestBody
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
