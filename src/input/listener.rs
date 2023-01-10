use crate::base::actions::Actions;
use crate::input::keymaps::{Actionable, KeyMap};
use crossterm::event::KeyCode;
use std::boxed::Box;
use std::rc::Rc;
use std::sync::Arc;

use crate::input::keymaps::default_keymap_factory;

//
// TODO:
// Currently, the code depends on 'KeyCode' interface of 'crossterm' library.
// But, it should depends on a abstration. Like the repository used in WebClient.
//

#[derive(Clone)]
pub struct KeyboardListerner {
    pub default: Arc<KeyMap>,
    pub current: Arc<KeyMap>,
}

impl KeyboardListerner {
    pub fn init(default_map: KeyMap) -> Self {
        let default_map = Arc::new(default_map);
        KeyboardListerner {
            default: default_map.clone(),
            current: default_map.clone(),
        }
    }

    pub fn get_command(&mut self, key: KeyCode) -> Option<Actions> {
        let binding = self.current.get(&key);

        if let Some(i) = binding.as_ref() {
            // If there is a subcommands it ignores the command and change
            // the state of current Keymap to the inside 'subcommands'

            if let Some(subcommands) = i.sub_action.clone() {
                self.current = Arc::new(subcommands);
                return Some(Actions::SubCommand);
            }

            // Otherwise... Return the command normaly
            let dd = i.action.clone();
            self.current = self.default.clone();
            return Some(dd);
        }

        // Anyway, reset to default keymap and return None
        self.current = self.default.clone();
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_init_and_be_defined() {
        let keymap = default_keymap_factory();

        // Basics Commands
        assert_eq!(
            keymap.get(&KeyCode::Char('k')),
            Some(&Actionable {
                action: Actions::Up,
                sub_action: None
            })
        );
    }

    #[test]
    fn should_get_command_of_single_keymaps() {
        let default_keymap = default_keymap_factory();
        let mut keymap = KeyboardListerner::init(default_keymap);

        let up = keymap.get_command(KeyCode::Char('k'));
        assert_eq!(up, Some(Actions::Up));
    }

    #[test]
    fn should_get_command_of_compound_keymaps() {
        let default_keymap = default_keymap_factory();
        let mut keymap = KeyboardListerner::init(default_keymap);

        let g = keymap.get_command(KeyCode::Char('g'));
        assert_eq!(g, Some(Actions::SubCommand));
        let g = keymap.get_command(KeyCode::Char('g'));
        assert_ne!(g, None);

        let g2 = keymap.get_command(KeyCode::Char('g'));
        assert_eq!(g2, Some(Actions::SubCommand));
        let g2 = keymap.get_command(KeyCode::Char('t'));
        assert_ne!(g2, None);
    }

    #[test]
    fn should_reset_keymap_when_a_undefined_key_is_pressed() {
        let default_keymap = default_keymap_factory();
        let mut keymap = KeyboardListerner::init(default_keymap);

        let g = keymap.get_command(KeyCode::Char('g'));
        assert_eq!(g, Some(Actions::SubCommand));

        // This is a undefined command
        let g = keymap.get_command(KeyCode::Char('_'));
        assert_eq!(g, None);

        // It should reset to default and execute normal commands
        let up = keymap.get_command(KeyCode::Char('k'));
        assert_eq!(up, Some(Actions::Up));
    }
}
