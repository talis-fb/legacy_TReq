use crate::actions::Actions;
use crate::states::{Commands, CommandsMap, State, StatesNames};
use std::collections::HashMap;

pub struct DefaultState {
    pub maps: CommandsMap,
}
impl State for DefaultState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::Default
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                // General Move
                (Actions::Up, Commands::go_to_tab_section()),
                (Actions::Down, Commands::go_to_log_section()),
                (Actions::Right, Commands::go_to_response_body_section()),
                (Actions::Left, Commands::go_to_request_body_section()),
                // Jumps
                (Actions::GoToNextTab, Commands::go_to_next_tab()),
                (Actions::GoToPreviousTab, Commands::go_to_previous_tab()),
                (Actions::GoToTabList, Commands::go_to_tab_section()),
                (
                    Actions::GoToRequestBody,
                    Commands::go_to_request_body_section(),
                ),
                (
                    Actions::GoToResponseBody,
                    Commands::go_to_response_body_section(),
                ),
                (Actions::GoToLogs, Commands::go_to_log_section()),
                (Actions::RenameTab, Commands::rename_tab()),
                (Actions::DeleteTab, Commands::delete_tab()),
            ]),
        }
    }
}
