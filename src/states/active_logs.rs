use crate::events::EVENTS;
use crate::states::{CommandFunc, CommandsList, Map, State, StatesNames};
use std::collections::HashMap;

pub struct TabActiveState {
    pub maps: Map,
}
impl State for TabActiveState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::Log
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (EVENTS::Edit, CommandsList::do_nothing()),
                (EVENTS::Switch, CommandsList::do_nothing()),
                (EVENTS::Up, CommandsList::go_to_request_body_section()),
                (EVENTS::Down, CommandsList::do_nothing()),
            ]),
        }
    }
}
