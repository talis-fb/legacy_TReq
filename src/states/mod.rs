use crate::app::App;
use crate::commands::{self, Command, CommandsList};
use crate::events::EVENTS;
use std::collections::HashMap;

type CommandFunc = fn(app: &mut App) -> Result<(), String>;

pub trait State {
    fn init() -> Self;
    fn get_command_of_event(&self, event: &EVENTS) -> Option<CommandFunc>;
}

// Default
pub struct DefaultState {
    maps: HashMap<EVENTS, CommandFunc>,
}
impl State for DefaultState {
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (EVENTS::Up, CommandsList::add_new_tab()),
                (EVENTS::Down, CommandsList::add_new_tab()),
                (EVENTS::Left, CommandsList::add_new_tab()),
            ]),
        }
    }
    fn get_command_of_event(&self, event: &EVENTS) -> Option<CommandFunc> {
        let command = self.maps.get(event)?;
        Some(*command)
    }
}
// ---------------------
// List of all State....
// ---------------------
pub struct TabActiveState {
    maps: HashMap<EVENTS, CommandFunc>,
}
impl State for TabActiveState {
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                (EVENTS::Edit, CommandsList::add_new_tab()),
                (EVENTS::Switch, CommandsList::add_new_tab()),
            ]),
        }
    }
    fn get_command_of_event(&self, event: &EVENTS) -> Option<CommandFunc> {
        let command = self.maps.get(event)?;
        Some(*command)
    }
}
