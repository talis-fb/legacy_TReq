use crate::actions::Actions;
use crate::base::states::states::{CommandsMap, State, StatesNames};
use crate::commands::Commands;
use std::collections::HashMap;

pub struct RequestHeaderActiveState {
    pub maps: CommandsMap,
}
impl State for RequestHeaderActiveState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::RequestHeaders
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (Actions::Edit, Commands::edit_request_headers_vim()),
                (Actions::Switch, Commands::go_to_request_body_section()),
                (Actions::Up, Commands::go_to_url_section()),
            ]),
        }
    }
}
