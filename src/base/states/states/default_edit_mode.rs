use crate::actions::Actions;
use crate::base::states::states::{CommandsMap, State, StatesNames};
use crate::commands::Commands;
use crate::input::keymaps::input_mode::keymap_factory;
use std::collections::HashMap;

pub struct DefaultEditMode {
    pub maps: CommandsMap,
}
impl State for DefaultEditMode {
    fn get_state_name(&self) -> StatesNames {
        StatesNames::DefaultEditMode
    }
    fn get_map(&self) -> &CommandsMap {
        &self.maps
    }
    fn init() -> Self {
        let mut maps = HashMap::from([
            // General Move
            (Actions::Up, Commands::go_to_tab_section()),
            (Actions::Down, Commands::go_to_tab_section()),
            (Actions::Right, Commands::go_to_response_body_section()),
            (Actions::Left, Commands::go_to_request_body_section()),
            (Actions::TypingClose, Commands::process_edit_mode()),
            (Actions::TypingCancel, Commands::cancel_edit_mode()),
            (Actions::TypingErase, Commands::erase_last_char_edit_mode()),
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
