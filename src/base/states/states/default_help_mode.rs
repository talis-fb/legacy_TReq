use crate::base::actions::Actions;
use crate::base::commands::Commands;
use crate::base::states::states::{CommandsMap, State, StatesNames};
use std::collections::HashMap;

pub struct DefaultHelpMode {
    pub maps: CommandsMap,
}
impl State for DefaultHelpMode {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::DefaultHelpMode
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        let maps = HashMap::from([
            // General Move
            (Actions::Up, Commands::doc_up()),
            (Actions::Down, Commands::doc_down()),
            (Actions::DocExit, Commands::doc_exit()),
        ]);

        Self { maps }
    }
}
