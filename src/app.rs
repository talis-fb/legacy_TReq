use crate::events::EVENTS;
use crate::states::{default::DefaultState, State};
use crossterm::event::KeyCode;
use std::collections::hash_map::HashMap;

use tui::layout::Rect;
use tui::widgets::Widget;

use crate::keymaps::KeyMap;
use crate::request::{Request, METHODS};

// #[derive(Default)]
pub struct App<'a> {
    request_history: Vec<Request>,
    pub current_request: usize,
    pub keymap: KeyMap<'a>,
    pub log: String,
    keys_queue: String,

    // States
    pub default_state: Box<dyn State>,
    pub current_state: Box<dyn State>,
}

impl App<'_> {
    pub fn init(keymap: KeyMap) -> App {
        App {
            request_history: vec![],
            current_request: 0,
            keymap,
            log: "".to_string(),
            keys_queue: "".to_string(),
            default_state: Box::new(DefaultState::init()),
            current_state: Box::new(DefaultState::init()),
        }
    }

    pub fn get_event_of_key(&mut self, key: KeyCode) -> Option<&EVENTS> {
        let event = self.keymap.get_command(key);

        // Manage the 'keys_queue' based in event received
        if let Some(EVENTS::SubCommand) = event {
            self.keys_queue.push('g');
        } else {
            self.keys_queue.clear();
        }

        event
    }

    pub fn get_requests(&self) -> &Vec<Request> {
        &self.request_history
    }

    pub fn get_current_request(&self) -> &Request {
        &self.request_history[self.current_request]
    }

    pub fn create_request(&mut self, req: Request) -> () {
        self.request_history.push(req);
    }

    pub fn set_current_request(&mut self, req: Request) -> () {
        self.request_history[self.current_request] = req;
    }

    pub fn set_log(&mut self, log: String) -> () {
        self.log = log;
    }

    pub fn get_keys_queue(&self) -> &String {
        &self.keys_queue
    }

    pub fn append_keys_queue(&mut self, ch: char) -> () {
        self.keys_queue.push(ch)
    }

    pub fn clear_keys_queue(&mut self) -> () {
        self.keys_queue = "".to_string()
    }
}
