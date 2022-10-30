use crate::app::App;
use crate::commands::{self, Command, CommandsList};
use crate::events::EVENTS;
use std::collections::HashMap;

type Func<'a> = fn(app: &'a mut App) -> Result<(), &'a str>;

pub struct BodyMap {
    maps: HashMap<EVENTS, Func<'static>>,
}

pub trait State<'a> {
    fn get_command_of_event(m: &'a BodyMap, event: &'a EVENTS) -> Option<Func<'static>>;
}

pub type StateMap = HashMap<EVENTS, Box<dyn Command>>;

// ---------------------
// List of all State....
// ---------------------

impl State<'_> for BodyMap {
    fn get_command_of_event<'a>(m: &'a BodyMap, event: &'a EVENTS) -> Option<Func<'static>> {
        let command = m.maps.get(event)?;
        Some(*command)
    }
}
