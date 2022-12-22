use crate::actions::Actions;
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
        StatesNames::Log
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (Actions::Edit, Commands::do_nothing()),
                (Actions::Switch, Commands::do_nothing()),
                (Actions::Up, Commands::go_to_request_body_section()),
                (Actions::Down, Commands::do_nothing()),
            ]),
        }
    }
}
