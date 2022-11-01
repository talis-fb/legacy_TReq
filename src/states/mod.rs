use crate::app::App;
use crate::commands::{self, CommandsList};
use crate::events::EVENTS;
use std::collections::HashMap;

type Map = HashMap<EVENTS, CommandFunc>;
type CommandFunc = fn(app: &mut App) -> Result<(), String>;

pub trait State {
    fn init() -> Self;
    fn get_command_of_event(maps: &Map, event: &EVENTS) -> Option<CommandFunc> {
        // println!("AAAA");
        let command = maps.get(event)?;
        // println!("BBBB");
        Some(*command)
    }
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
                // (EVENTS::Left, CommandsList::add_new_tab()),
            ]),
        }
    }
}
// ---------------------
// List of all State....
// ---------------------
pub struct TabActiveState {
    maps: Map,
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
