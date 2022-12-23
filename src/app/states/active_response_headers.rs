use crate::actions::Actions;
use crate::states::{Commands, CommandsMap, State, StatesNames};
use std::collections::HashMap;

pub struct ResponseHeadersState {
    pub maps: CommandsMap,
}
impl State for ResponseHeadersState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::ResponseHeader
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
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
