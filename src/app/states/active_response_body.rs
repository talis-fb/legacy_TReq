use crate::actions::Actions;
use crate::states::{CommandFunc, Commands, Map, State, StatesNames};
use std::collections::HashMap;

pub struct RequestBodyActiveState {
    pub maps: Map,
}
impl State for RequestBodyActiveState {
    fn get_map(&self) -> &Map {
        &self.maps
    }
    fn get_state_name(&self) -> StatesNames {
        StatesNames::ResponseBody
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (Actions::Edit, Commands::do_nothing()),
                (Actions::Switch, Commands::do_nothing()),
            ]),
        }
    }
}
