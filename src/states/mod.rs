use crate::app::App;
use crate::commands::{self, CommandsList};
use crate::events::EVENTS;
use std::collections::HashMap;

type Map = HashMap<EVENTS, CommandFunc>;
type CommandFunc = fn(app: &mut App) -> Result<(), String>;

pub trait State {
    fn init() -> Self;
}

pub fn get_command_of_event_with_states(maps: Vec<&Map>, event: &EVENTS) -> Option<CommandFunc> {
    for m in maps.iter() {
        if let Some(command) = m.get(event) {
            return Some(*command);
        }
    }
    Some(CommandsList::do_nothing())
}
pub fn get_command_of_event(maps: &Map, event: &EVENTS) -> Option<CommandFunc> {
    let command = maps.get(event)?;
    Some(*command)
}

// Default
pub struct DefaultState {
    pub maps: Map,
}
impl State for DefaultState {
    fn init() -> Self {
        Self {
            maps: HashMap::from([
                // (EVENTS::Up, CommandsList::add_new_tab()),
                (EVENTS::GoToNextTab, CommandsList::go_to_next_tab()),
                (EVENTS::GoToPreviousTab, CommandsList::go_to_previous_tab()),
                // (EVENTS::Left, CommandsList::add_new_tab()),
            ]),
        }
    }
}
// ---------------------
// List of all State....
// ---------------------
pub struct TabActiveState {
    pub maps: Map,
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
}
