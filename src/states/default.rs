use crate::events::EVENTS;
use crate::states::{CommandFunc, CommandsList as Commands, Map, State, StatesNames};
use std::collections::HashMap;

pub struct DefaultState {
    pub maps: Map,
}
impl State for DefaultState {
    fn get_map(&self) -> &Map {
        &self.maps
    }
    fn get_state_name(&self) -> StatesNames {
        StatesNames::Default
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                // General Move
                (EVENTS::Up, Commands::go_to_tab_section()),
                (EVENTS::Down, Commands::go_to_log_section()),
                (EVENTS::Right, Commands::go_to_response_body_section()),
                (EVENTS::Left, Commands::go_to_request_body_section()),
                // Jumps
                (EVENTS::GoToNextTab, Commands::go_to_next_tab()),
                (EVENTS::GoToPreviousTab, Commands::go_to_previous_tab()),
                (EVENTS::GoToTabList, Commands::go_to_tab_section()),
                (
                    EVENTS::GoToRequestBody,
                    Commands::go_to_request_body_section(),
                ),
                (
                    EVENTS::GoToResponseBody,
                    Commands::go_to_response_body_section(),
                ),
                (EVENTS::GoToLogs, Commands::go_to_log_section()),
                (EVENTS::RenameTab, Commands::rename_tab()),
                (EVENTS::DeleteTab, Commands::delete_tab()),
            ]),
        }
    }
}
