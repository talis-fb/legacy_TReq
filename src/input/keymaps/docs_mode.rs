use crate::base::actions::Actions;
use crossterm::event::KeyCode;
use std::collections::HashMap;

use super::utils::{create_keymap, create_keymap_char};
use super::KeyMap;

pub fn keymap_factory() -> KeyMap {
    HashMap::from([
        create_keymap(KeyCode::Up, Actions::Up),
        create_keymap(KeyCode::Down, Actions::Down),
        create_keymap_char('k', Actions::Up),
        create_keymap_char('j', Actions::Down),
        // All rest
        create_keymap_char('l', Actions::DocExit),
        create_keymap_char('h', Actions::DocExit),
        create_keymap_char('?', Actions::DocExit),
        create_keymap_char('q', Actions::DocExit),
        create_keymap_char('e', Actions::DocExit),
        create_keymap_char('d', Actions::DocExit),
        create_keymap_char('n', Actions::DocExit),
        create_keymap_char('r', Actions::DocExit),
        create_keymap_char('s', Actions::DocExit),
        create_keymap_char('G', Actions::DocExit),
    ])
}
