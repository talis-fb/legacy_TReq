use crate::base::actions::Actions;
use crate::base::commands::Commands;
use crate::base::states::states::{CommandsMap, State, StatesNames};
use std::collections::HashMap;

pub struct RequestActiveState {
    pub maps: CommandsMap,
}
impl State for RequestActiveState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::RequestBody
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (Actions::Edit, Commands::edit_request_body_vim()),
                (Actions::Switch, Commands::go_to_request_header_section()),
                (Actions::Up, Commands::go_to_url_section()),
                (Actions::Right, Commands::go_to_response_body_section()),
            ]),
        }
    }
}
