use crate::actions::Actions;
use crate::base::states::states::{CommandsMap, State, StatesNames};
use crate::commands::Commands;
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
                (Actions::Down, Commands::go_to_url_section()),
            ]),
        }
    }
}
