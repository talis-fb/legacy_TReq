use std::collections::hash_map::HashMap;

use tui::layout::Rect;
use tui::widgets::Widget;

use crate::keymaps::KeyMap;
use crate::request::{Request, METHODS};

// #[derive(Default)]
pub struct App<'a> {
    request_history: Vec<Request>,
    pub current_request: usize,
    pub keymap: &'a KeyMap<'a>,
}

impl App<'_> {
    pub fn init<'a>(keymap: &'a KeyMap) -> App<'a> {
        App {
            request_history: vec![],
            current_request: 0,
            keymap,
        }
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
}

// STATE MANAGENT
