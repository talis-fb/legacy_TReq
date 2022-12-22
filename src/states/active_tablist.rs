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
        StatesNames::TabList
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (Actions::Edit, Commands::rename_tab()),
                (Actions::Switch, Commands::go_to_next_tab()),
                (Actions::New, Commands::add_new_tab()),
                (Actions::Up, Commands::do_nothing()),
                (Actions::Down, Commands::go_to_url_section()),
            ]),
        }
    }
}
