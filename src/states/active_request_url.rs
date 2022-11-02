use crate::events::EVENTS;
use crate::states::{CommandFunc, CommandsList, Map, State, StatesNames};
use std::collections::HashMap;

pub struct TabActiveState {
    pub maps: Map,
}
impl State for TabActiveState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::Url
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (EVENTS::Edit, CommandsList::go_to_log_section()),
                (EVENTS::Switch, CommandsList::go_to_next_tab()),
                (EVENTS::New, CommandsList::add_new_tab()),
                (EVENTS::Up, CommandsList::go_to_tab_section()),
                (EVENTS::Down, CommandsList::go_to_request_body_section()),
            ]),
        }
    }
}
