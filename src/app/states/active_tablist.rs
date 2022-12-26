use crate::actions::Actions;
use crate::states::{Commands, CommandsMap, State, StatesNames};
use std::collections::HashMap;

pub struct TabActiveState {
    pub maps: CommandsMap,
}
impl State for TabActiveState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::TabList
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (Actions::Edit, Commands::rename_tab()),
                (Actions::Switch, Commands::go_to_next_tab()),
                (Actions::New, Commands::add_new_tab()),
                (Actions::Up, Commands::do_nothing()),
                (Actions::Right, Commands::go_to_response_body_section()),
                (Actions::Left, Commands::go_to_url_section()),
                (Actions::Down, Commands::go_to_url_section()),
            ]),
        }
    }
}
