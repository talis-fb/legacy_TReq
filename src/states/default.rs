use crate::events::EVENTS;
use crate::states::{CommandFunc, CommandsList, Map, State, StatesNames};
use std::collections::HashMap;

pub struct DefaultState {
    pub maps: Map,
}
impl State for DefaultState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::Default
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (EVENTS::Up, CommandsList::go_to_tab_section()),
                (EVENTS::Down, CommandsList::go_to_tab_section()),
                (EVENTS::Right, CommandsList::go_to_tab_section()),
                (EVENTS::Left, CommandsList::go_to_tab_section()),
                (EVENTS::GoToNextTab, CommandsList::go_to_next_tab()),
                (EVENTS::GoToPreviousTab, CommandsList::go_to_previous_tab()),
                (EVENTS::GoToTabList, CommandsList::go_to_tab_section()),
                // (EVENTS::Left, CommandsList::add_new_tab()),
            ]),
        }
    }
}
