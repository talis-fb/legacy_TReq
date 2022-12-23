use crate::actions::Actions;
use crate::states::{Commands, CommandsMap, State, StatesNames};
use std::collections::HashMap;

pub struct RequestUrlActiveState {
    pub maps: CommandsMap,
}
impl State for RequestUrlActiveState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::RequestHeaders
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (Actions::Edit, Commands::edit_request_url()),
                (Actions::Switch, Commands::go_to_next_tab()),
                (Actions::New, Commands::add_new_tab()),
                (Actions::Up, Commands::go_to_tab_section()),
                (Actions::Down, Commands::go_to_request_body_section()),
            ]),
        }
    }
}
