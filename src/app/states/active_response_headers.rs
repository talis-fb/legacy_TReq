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
                (Actions::Switch, Commands::go_to_response_body_section()),
                (Actions::Left, Commands::go_to_url_section()),
                (Actions::Up, Commands::go_to_tab_section()),
                (Actions::Down, Commands::go_to_log_section()),
            ]),
        }
    }
}
