use crate::events::EVENTS;
use crate::states::{CommandFunc, CommandsList, Map, State, StatesNames};
use std::collections::HashMap;

pub struct TabActiveState {
    pub maps: Map,
}
impl State for TabActiveState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::TabActive
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (EVENTS::Edit, CommandsList::add_new_tab()),
                (EVENTS::Switch, CommandsList::go_to_next_tab()),
                (EVENTS::New, CommandsList::add_new_tab()),
            ]),
        }
    }
}
