use crossterm::event::{self, Event, KeyCode};

use crate::{
    app::app::App,
    base::{
        commands::{Command, Commands},
        store::DataStore,
    },
    view::ui::UI,
};

pub trait InputBuffer {
    fn block_reading(&mut self) -> Result<String, ()>;
}

#[derive(Clone)]
pub struct InputKeyboardBuffer {
    pub buffer: String,
    pub command: Command,
}
impl InputKeyboardBuffer {
    pub fn init() -> Self {
        InputKeyboardBuffer {
            buffer: String::new(),
            command: Commands::do_nothing(),
        }
    }
}
