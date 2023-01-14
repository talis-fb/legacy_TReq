use crossterm::event::{self, Event, KeyCode};

use crate::{
    app::app::App,
    base::{
        commands::{Command, Commands},
        store::MainStore,
    },
    view::ui::UI,
};

pub trait InputBuffer {
    fn block_reading(&mut self) -> Result<String, ()>;
}

#[derive(Clone)]
pub struct InputKeyboardBuffer {
    pub value_backup: Option<String>,
    pub value: String,
    pub command: Command,
}
impl InputKeyboardBuffer {
    pub fn init() -> Self {
        InputKeyboardBuffer {
            value: String::new(),
            command: Commands::do_nothing(),
            value_backup: None,
        }
    }
    pub fn set_backup(&mut self, s: String) -> () {
        self.value_backup = Some(s)
    }
    pub fn reset_to_backup(&mut self) -> () {
        self.value = self.value_backup.clone().unwrap_or(String::new())
    }
}
