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
                (EVENTS::Edit, CommandsList::rename_tab()),
                (EVENTS::Switch, CommandsList::go_to_next_tab()),
                (EVENTS::New, CommandsList::add_new_tab()),
            ]),
        }
    }
}
