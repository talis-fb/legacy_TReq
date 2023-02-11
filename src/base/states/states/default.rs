use crate::actions::Actions;
use crate::base::states::states::{CommandsMap, State, StatesNames};
use crate::commands::Commands;
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
                (Actions::Down, Commands::go_to_tab_section()),
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
                (Actions::Submit, Commands::submit()),
                (Actions::Quit, Commands::quit()),
                (Actions::AskForHelp, Commands::open_help_screen()),
                (Actions::Save, Commands::save_request()),
                (Actions::GrowHorizontalUiLeft, Commands::grow_left_ui()),
                (Actions::GrowHorizontalUiRight, Commands::grow_right_ui()),
                (Actions::ReloadBody, Commands::restart_body_of_file()),
            ]),
        }
    }
}
