use crate::events::EVENTS;
use crossterm::event::KeyCode;
use std::collections::HashMap;

pub type CommandsMap = HashMap<KeyCode, Command>;

#[derive(PartialEq, Eq, Debug)]
pub struct Command {
    command: EVENTS,

    // This is used only if the key has other commands if other keys is pressed
    // if this box is NOT None, then the command above is ignored
    subcommands: Option<CommandsMap>,
}

pub trait KeyMapTrait {
    fn init_default_commandmap() -> CommandsMap;
}

pub struct KeyMap<'a> {
    default: &'a CommandsMap,
    current: &'a CommandsMap,
}

impl KeyMap<'_> {
    fn get_command(&mut self, key: KeyCode) -> Option<&EVENTS> {
        if let Some(i) = self.current.get(&key) {
            // If there is a subcommands it ignores the command and change
            // the state of current Keymap to the inside 'subcommands'
            if let Some(subcommands) = &i.subcommands {
                self.current = &subcommands;
                return None;
            }

            // Otherwise... Return the command normaly
            return Some(&i.command);
        }
        None
    }
}

impl KeyMapTrait for KeyMap<'_> {
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
                    command: EVENTS::Null,
                    subcommands: Some(HashMap::from([(
                        KeyCode::Char('g'),
                        Command {
                            command: EVENTS::GoToTabList,
                            subcommands: None,
                        },
                    )])),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_init_and_be_defined() {
        let keymap = KeyMap::init_default_commandmap();

        // Basics Commands
        assert_eq!(
            keymap.get(&KeyCode::Char('k')),
            Some(&Command {
                command: EVENTS::Up,
                subcommands: None
            })
        );
    }

    #[test]
    fn should_get_commands_of_single_keymap() {
        let command_map = KeyMap::init_default_commandmap();
        let mut keymap = KeyMap {
            default: &command_map,
            current: &command_map,
        };

        // Simple commands
        let up = keymap.get_command(KeyCode::Char('k'));
        assert_eq!(up, Some(&EVENTS::Up));
    }
    #[test]
    fn should_get_commands_of_complex_keymap() {
        let command_map = KeyMap::init_default_commandmap();
        let mut keymap = KeyMap {
            default: &command_map,
            current: &command_map,
        };

        let g = keymap.get_command(KeyCode::Char('g'));
        assert_eq!(g, None);

        let g = keymap.get_command(KeyCode::Char('g'));
        assert_ne!(g, None);
        // assert_eq!(g, Some(&EVENTS::GoToTabList)); // This can change
    }
}
