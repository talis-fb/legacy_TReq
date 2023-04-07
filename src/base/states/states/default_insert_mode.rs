use crate::actions::Actions;
use crate::base::states::states::{CommandsMap, State, StatesNames};
use crate::commands::Commands;
use crate::input::keymaps::insert_mode::keymap_factory;
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
            (Actions::Right, Commands::edit_mode_go_to_next_char()),
            (Actions::Left, Commands::edit_mode_go_to_prev_char()),
            (Actions::TypingErase, Commands::edit_mode_delete_prev_char()),
            (Actions::TypingBegingLine, Commands::edit_mode_go_to_start()),
            (Actions::TypingEndLine, Commands::edit_mode_go_to_end()),
            (Actions::TypingClose, Commands::process_edit_mode()),
            (Actions::TypingCancel, Commands::cancel_edit_mode()),
        ]);

        keymap_factory().values().for_each(|f| {
            let action = f.action;
            if let Actions::TypingChar(ch) = action {
                maps.insert(action, Commands::edit_mode_insert_char(ch));
            };
        });

        Self { maps }
    }
}
