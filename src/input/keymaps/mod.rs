use crate::base::actions::Actions;
use crossterm::event::KeyCode;
use std::collections::HashMap;

pub mod docs_mode;
pub mod input_mode;
pub mod normal_mode;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Actionable {
    pub action: Actions,

    // This is used only if the key has other commands if other keys is pressed
    // if this box is NOT None, then the command above is ignored
    pub sub_action: Option<KeyMap>,
}

pub type KeyMap = HashMap<KeyCode, Actionable>;

//
// What to do...
// * When go to input/docs/vim mode it should change Keymap in InputHandler
// and also go to a State to handle Actions of typing, ONLY this
// (this state can be optional, possible)
//
// The new keymap should overwrite evething to only actions specials of that mode
//
// The app will receive all them in the same way. The only change will be in InputHandler
// with Keymap used. With this, it will send Actions in the same way to App

mod utils {
    use super::*;

    pub fn create_keymap_char(key: char, action: Actions) -> (KeyCode, Actionable) {
        (
            KeyCode::Char(key),
            Actionable {
                action,
                sub_action: None,
            },
        )
    }

    pub fn create_sub_keymap_char(key: char, subcommands: KeyMap) -> (KeyCode, Actionable) {
        (
            KeyCode::Char(key),
            Actionable {
                action: Actions::SubCommand,
                sub_action: Some(subcommands),
            },
        )
    }

    pub fn create_keymap(key_code: KeyCode, action: Actions) -> (KeyCode, Actionable) {
        (
            key_code,
            Actionable {
                action,
                sub_action: None,
            },
        )
    }
}
