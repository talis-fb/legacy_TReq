use crate::app::App;
use crate::commands::{self, CommandsList};
use crate::events::EVENTS;
use std::collections::HashMap;

pub mod active_request_body;
pub mod active_request_headers;
pub mod active_request_url;
pub mod active_response_body;
pub mod active_response_headers;
pub mod active_tablist;
pub mod default;

pub type Map = HashMap<EVENTS, CommandFunc>;
pub type CommandFunc = fn(app: &mut App) -> Result<(), String>;

#[derive(PartialEq, Eq, Clone)]
pub enum StatesNames {
    TabActive,
    Default,
    Request,
    RequestHeader,
    RequestBody,
    ResponseHeader,
    ResponseBody,
}

pub trait State {
    fn get_state_name(&self) -> StatesNames;
    fn init() -> Self
    where
        Self: Sized;
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
