use crate::actions::Actions;
use crate::states::{Commands, CommandsMap, State, StatesNames};
use std::collections::HashMap;

pub struct ResponseBodyActiveState {
    pub maps: CommandsMap,
}
impl State for ResponseBodyActiveState {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::ResponseBody
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (Actions::Edit, Commands::do_nothing()),
                (Actions::Switch, Commands::do_nothing()),
                (Actions::Left, Commands::go_to_url_section()),
                (Actions::Up, Commands::go_to_tab_section()),
                (Actions::Down, Commands::go_to_log_section()),
            ]),
        }
    }
}
