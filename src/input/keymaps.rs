use crate::base::actions::Actions;
use crossterm::event::KeyCode;
use std::collections::HashMap;

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
// and ONLY this
//
// The new keymap should overwrite evething to only actions specials of that mode
//
// The app will receive all them in the same way. The only change will be in InputHandler
// with Keymap used. With this, it will send Actions in the same way to App

pub fn default_keymap_factory() -> KeyMap {
    HashMap::from([
        create_keymap_char('?', Actions::AskForHelp),
        create_keymap_char('q', Actions::Quit),
        create_keymap_char('e', Actions::Edit),
        create_keymap_char('d', Actions::Delete),
        create_keymap(KeyCode::Enter, Actions::Submit),
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

fn create_keymap_char(key: char, action: Actions) -> (KeyCode, Actionable) {
    (
        KeyCode::Char(key),
        Actionable {
            action,
            sub_action: None,
        },
    )
}

fn create_sub_keymap_char(key: char, subcommands: KeyMap) -> (KeyCode, Actionable) {
    (
        KeyCode::Char(key),
        Actionable {
            action: Actions::Null,
            sub_action: Some(subcommands),
        },
    )
}

fn create_keymap(key_code: KeyCode, action: Actions) -> (KeyCode, Actionable) {
    (
        key_code,
        Actionable {
            action,
            sub_action: None,
        },
    )
}
