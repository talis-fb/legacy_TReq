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
    None
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
                (EVENTS::Up, CommandsList::err()),
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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn should_get_commands_of_a_map() {
//         let state = DefaultState::init();
//         let event = EVENTS::GoToNextTab;
//         let command = get_command_of_event(&state.maps, &event).unwrap();
//         let op = CommandsList::go_to_next_tab();
//         assert!(
//             // get_command_of_event(&state.maps, &event).unwrap() as CommandFunc,
//             CommandsList::go_to_next_tab() as CommandFunc
//                 == CommandsList::go_to_next_tab() as CommandFunc // state.maps.get(&event).unwrap()
//         );
//     }
// }
