use crate::base::actions::Actions;
use crossterm::event::KeyCode;
use std::collections::HashMap;

use super::utils::{create_keymap, create_keymap_char, create_sub_keymap_char};
use super::KeyMap;

pub fn keymap_factory() -> KeyMap {
    HashMap::from([
        create_keymap_char('?', Actions::AskForHelp),
        create_keymap_char('q', Actions::Quit),
        create_keymap_char('e', Actions::Edit),
        create_keymap_char('d', Actions::Delete),
        create_keymap(KeyCode::Enter, Actions::Submit),
        create_keymap(KeyCode::Esc, Actions::CancelSubmit),
        create_keymap(KeyCode::Tab, Actions::Switch),
        create_keymap(KeyCode::BackTab, Actions::InverseSwitch),
        create_keymap_char('j', Actions::Down),
        create_keymap_char('k', Actions::Up),
        create_keymap_char('l', Actions::Right),
        create_keymap_char('h', Actions::Left),
        create_keymap(KeyCode::Up, Actions::Up),
        create_keymap(KeyCode::Down, Actions::Down),
        create_keymap(KeyCode::Left, Actions::Left),
        create_keymap(KeyCode::Right, Actions::Right),
        create_keymap_char('n', Actions::New),
        create_keymap_char('r', Actions::ReloadBody),
        create_keymap_char('u', Actions::Undo),
        create_keymap_char('s', Actions::Save),
        create_keymap_char('G', Actions::GoToLogs),
        create_sub_keymap_char(
            'g',
            HashMap::from([
                create_keymap_char('g', Actions::GoToTabList),
                create_keymap_char('t', Actions::GoToNextTab),
                create_keymap_char('T', Actions::GoToPreviousTab),
                create_keymap_char('l', Actions::GrowHorizontalUiLeft),
                create_keymap_char('h', Actions::GrowHorizontalUiRight),
            ]),
        ),
    ])
}
