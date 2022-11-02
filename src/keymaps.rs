use crate::events::EVENTS;
use crossterm::event::KeyCode;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Command {
    command: EVENTS,

    // This is used only if the key has other commands if other keys is pressed
    // if this box is NOT None, then the command above is ignored
    subcommands: Option<CommandsMap>,
}

pub type CommandsMap = HashMap<KeyCode, Command>;

#[derive(Clone)]
pub struct KeyMap<'a> {
    pub default: &'a CommandsMap,
    pub current: &'a CommandsMap,
}

impl KeyMap<'_> {
    pub fn get_command(&mut self, key: KeyCode) -> Option<&EVENTS> {
        if let Some(i) = self.current.get(&key) {
            // If there is a subcommands it ignores the command and change
            // the state of current Keymap to the inside 'subcommands'
            if let Some(subcommands) = &i.subcommands {
                self.current = &subcommands;
                return Some(&EVENTS::SubCommand);
            }

            // Otherwise... Return the command normaly
            self.current = self.default;
            return Some(&i.command);
        }
        // Anyway, it reset to default keymap and return None
        self.current = self.default;
        None
    }

    /// Generate the default KeyMap setting
    pub fn default_commandmap() -> CommandsMap {
        HashMap::from([
            (
                KeyCode::Enter,
                Command {
                    command: EVENTS::Submit,
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
            (
                KeyCode::Tab,
                Command {
                    command: EVENTS::Switch,
                    subcommands: None,
                },
            ),
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
                    subcommands: Some(HashMap::from([
                        (
                            KeyCode::Char('g'),
                            Command {
                                command: EVENTS::GoToTabList,
                                subcommands: None,
                            },
                        ),
                        (
                            KeyCode::Char('t'),
                            Command {
                                command: EVENTS::GoToNextTab,
                                subcommands: None,
                            },
                        ),
                        (
                            KeyCode::Char('T'),
                            Command {
                                command: EVENTS::GoToPreviousTab,
                                subcommands: None,
                            },
                        ),
                    ])),
                },
            ),
            (
                KeyCode::Char('G'),
                Command {
                    command: EVENTS::GoToLogs,
                    subcommands: None,
                },
            ),
            (
                KeyCode::Char('n'),
                Command {
                    command: EVENTS::New,
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
        let keymap = KeyMap::default_commandmap();

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
    fn should_get_command_of_single_keymaps() {
        let command_map = KeyMap::default_commandmap();
        let mut keymap = KeyMap {
            default: &command_map,
            current: &command_map,
        };

        // Simple commands
        let up = keymap.get_command(KeyCode::Char('k'));
        assert_eq!(up, Some(&EVENTS::Up));
    }

    #[test]
    fn should_get_command_of_compound_keymaps() {
        let command_map = KeyMap::default_commandmap();
        let mut keymap = KeyMap {
            default: &command_map,
            current: &command_map,
        };

        let g = keymap.get_command(KeyCode::Char('g'));
        assert_eq!(g, Some(&EVENTS::SubCommand));
        let g = keymap.get_command(KeyCode::Char('g'));
        assert_ne!(g, None);

        let g2 = keymap.get_command(KeyCode::Char('g'));
        assert_eq!(g2, Some(&EVENTS::SubCommand));
        let g2 = keymap.get_command(KeyCode::Char('t'));
        assert_ne!(g2, None);
    }

    #[test]
    fn should_reset_keymap_when_a_undefined_key_is_pressed() {
        let command_map = KeyMap::default_commandmap();
        let mut keymap = KeyMap {
            default: &command_map,
            current: &command_map,
        };

        let g = keymap.get_command(KeyCode::Char('g'));
        assert_eq!(g, Some(&EVENTS::SubCommand));

        // This is a undefined command
        let g = keymap.get_command(KeyCode::Char('_'));
        assert_eq!(g, None);

        // It should reset to default and execute normal commands
        let up = keymap.get_command(KeyCode::Char('k'));
        assert_eq!(up, Some(&EVENTS::Up));
    }
}