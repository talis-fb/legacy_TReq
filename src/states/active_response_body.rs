use crate::events::EVENTS;
use crate::states::{CommandFunc, CommandsList, Map, State, StatesNames};
use std::collections::HashMap;

pub struct RequestBodyActiveState {
    pub maps: Map,
}
impl State for RequestBodyActiveState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::ResponseBody
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (EVENTS::Edit, CommandsList::do_nothing()),
                (EVENTS::Switch, CommandsList::do_nothing()),
            ]),
        }
    }
}
