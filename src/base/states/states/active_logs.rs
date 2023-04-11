use crate::base::actions::Actions;
use crate::base::states::states::{CommandsMap, State, StatesNames};
use crate::base::commands::Commands;
use std::collections::HashMap;

pub struct LogsState {
    pub maps: CommandsMap,
}
impl State for LogsState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::Log
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
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
