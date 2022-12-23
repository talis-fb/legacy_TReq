use crate::actions::Actions;
use crate::states::{Commands, CommandsMap, State, StatesNames};
use std::collections::HashMap;

pub struct EmptyState {
    pub maps: CommandsMap,
}
impl State for EmptyState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::Empty
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([]),
        }
    }
}
