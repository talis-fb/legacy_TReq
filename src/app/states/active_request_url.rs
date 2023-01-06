use crate::actions::Actions;
use crate::states::{Commands, CommandsMap, State, StatesNames};
use std::collections::HashMap;

pub struct RequestUrlActiveState {
    pub maps: CommandsMap,
}
impl State for RequestUrlActiveState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::Url
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (Actions::Up, Commands::go_to_tab_section()),
                (Actions::Down, Commands::go_to_request_body_section()),
                (Actions::Right, Commands::go_to_response_body_section()),
                (Actions::Edit, Commands::edit_request_url()),
                (Actions::New, Commands::add_new_tab()),
                (Actions::Switch, Commands::switch_request_method()),
            ]),
        }
    }
}
