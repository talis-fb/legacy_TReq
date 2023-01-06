use crate::base::actions::Actions;
use crossterm::event::KeyCode;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Actionable {
    pub action: Actions,

    // This is used only if the key has other commands if other keys is pressed
    // if this box is NOT None, then the command above is ignored
    pub sub_action: Option<KeyMap>,
}

pub type KeyMap = HashMap<KeyCode, Actionable>;

pub fn default_keymap_factory() -> KeyMap {
    HashMap::from([
        (
            KeyCode::Enter,
            Actionable {
                action: Actions::Submit,
                sub_action: None,
            },
        ),
        (
            KeyCode::Char('q'),
            Actionable {
                action: Actions::Quit,
                sub_action: None,
            },
        ),
        (
            KeyCode::Char('e'),
            Actionable {
                action: Actions::Edit,
                sub_action: None,
            },
        ),
        (
            KeyCode::Char('s'),
            Actionable {
                action: Actions::Switch,
                sub_action: None,
            },
        ),
        (
            KeyCode::Char('j'),
            Actionable {
                action: Actions::Down,
                sub_action: None,
            },
        ),
        (
            KeyCode::Char('k'),
            Actionable {
                action: Actions::Up,
                sub_action: None,
            },
        ),
        (
            KeyCode::Char('l'),
            Actionable {
                action: Actions::Right,
                sub_action: None,
            },
        ),
        (
            KeyCode::Char('h'),
            Actionable {
                action: Actions::Left,
                sub_action: None,
            },
        ),
        (
            KeyCode::Char('g'),
            Actionable {
                action: Actions::Null,
                sub_action: Some(HashMap::from([
                    (
                        KeyCode::Char('g'),
                        Actionable {
                            action: Actions::GoToTabList,
                            sub_action: None,
                        },
                    ),
                    (
                        KeyCode::Char('t'),
                        Actionable {
                            action: Actions::GoToNextTab,
                            sub_action: None,
                        },
                    ),
                    (
                        KeyCode::Char('T'),
                        Actionable {
                            action: Actions::GoToPreviousTab,
                            sub_action: None,
                        },
                    ),
                ])),
            },
        ),
        (
            KeyCode::Char('G'),
            Actionable {
                action: Actions::GoToLogs,
                sub_action: None,
            },
        ),
        (
            KeyCode::Char('n'),
            Actionable {
                action: Actions::New,
                sub_action: None,
            },
        ),
    ])
}
