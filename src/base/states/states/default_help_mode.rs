use crate::actions::Actions;
use crate::base::states::states::{CommandsMap, State, StatesNames};
use crate::commands::Commands;
use crate::input::keymaps::input_mode::keymap_factory;
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
        let mut maps = HashMap::from([
            // General Move
            (Actions::Up, Commands::doc_up()),
            (Actions::Down, Commands::doc_down()),
            (Actions::DocExit, Commands::doc_exit()),
        ]);

        keymap_factory().values().for_each(|f| {
            let action = f.action;
            if let Actions::TypingChar(ch) = action {
                maps.insert(action, Commands::type_char_edit_mode(ch));
            };
        });

        Self { maps }
    }
}
