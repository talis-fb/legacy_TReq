use crate::events::EVENTS;
use crossterm::event::KeyCode;
use std::collections::HashMap;

pub type CommandsMap = HashMap<KeyCode, Command>;

pub struct Command {
    command: EVENTS,

    // This is used only if the key has other commands if other keys is pressed
    // if this box is NOT None, then the command above is ignored
    subcommands: Option<Box<CommandsMap>>,
}

pub trait KeyMapTrait {
    fn get(c: char) -> EVENTS;
    fn init_default_commandmap() -> CommandsMap;
}

pub struct KeyMap {
    default: Box<CommandsMap>,
    current: Box<CommandsMap>,
}

impl KeyMapTrait for KeyMap {
    fn get(c: char) -> EVENTS {
        // TODO: Make it work
        EVENTS::Down
    }
    fn init_default_commandmap() -> CommandsMap {
        HashMap::from([
            (
                KeyCode::Char('j'),
                Command {
                    command: EVENTS::Down,
                    subcommands: None,
                },
            ),
            (
                KeyCode::Char('k'),
                Command {
                    command: EVENTS::Up,
                    subcommands: None,
                },
            ),
            (
                KeyCode::Char('l'),
                Command {
                    command: EVENTS::Right,
                    subcommands: None,
                },
            ),
            (
                KeyCode::Char('h'),
                Command {
                    command: EVENTS::Left,
                    subcommands: None,
                },
            ),
            (
                KeyCode::Char('g'),
                Command {
                    subcommands: Some(Box::from(HashMap::from([(
                        KeyCode::Char('g'),
                        Command {
                            command: EVENTS::GoToTabList,
                            subcommands: None,
                        },
                    )]))),
                    command: EVENTS::Null,
                },
            ),
            (
                KeyCode::Tab,
                Command {
                    command: EVENTS::Edit,
                    subcommands: None,
                },
            ),
            (
                KeyCode::Char('e'),
                Command {
                    command: EVENTS::Edit,
                    subcommands: None,
                },
            ),
        ])
    }
}
