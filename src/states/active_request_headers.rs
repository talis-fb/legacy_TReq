use crate::events::EVENTS;
use crate::states::{CommandFunc, CommandsList, Map, State, StatesNames};
use std::collections::HashMap;

pub struct TabActiveState {
    pub maps: Map,
}
impl State for TabActiveState {
    fn get_map(&self) -> &Map {
        &self.maps
    }
    fn get_state_name(&self) -> StatesNames {
        StatesNames::RequestHeaders
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (EVENTS::Edit, CommandsList::edit_request_body()),
                (EVENTS::Switch, CommandsList::switch_request_options()),
                (EVENTS::Up, CommandsList::go_to_url_section()),
                (EVENTS::Down, CommandsList::go_to_log_section()),
            ]),
        }
    }
}
