use crate::base::actions::Actions;
use crate::states::{default::DefaultState, State};
use crossterm::event::KeyCode;
use std::collections::hash_map::HashMap;

use tui::layout::Rect;
use tui::widgets::Widget;

use crate::input::keymaps::KeyMap;
use crate::base::web::request::{Request, METHODS};

use crate::input::input::InputKeyboardBuffer;

#[derive(Clone)]
pub enum InputMode {
    Normal,
    Insert,
}

// #[derive(Default)]
pub struct App<'a> {
    request_history: Vec<Request>,
    pub current_request: usize,
    pub keymap: KeyMap<'a>,
    pub log: String,
    keys_queue: String,
    mode: InputMode,
    input_buffer: InputKeyboardBuffer,

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
            mode: InputMode::Normal,
            input_buffer: InputKeyboardBuffer::init(),
        }
    }

    // Input Mode
    pub fn get_mode(&self) -> InputMode {
        self.mode.clone()
    }
    pub fn set_input_mode_with_callback(&mut self, callback: fn(&mut App, String)) {
        self.input_buffer.set_callback(callback);
        self.mode = InputMode::Insert;
    }
    pub fn get_text_input_mode(&self) -> String {
        self.input_buffer.buffer.clone()
    }
    pub fn edit_input_mode(&mut self, key: KeyCode) {
        match key {
            KeyCode::Enter => {
                let callback = self.input_buffer.on_close;
                let content = self.input_buffer.buffer.clone();
                callback(self, content);

                // Reset Buffer
                self.input_buffer.buffer = String::new();

                // Come back to normal mode
                self.mode = InputMode::Normal;
            }
            KeyCode::Backspace => {
                self.input_buffer.pop_char();
            }
            KeyCode::Char(i) => {
                self.input_buffer.append_char(i);
            }
            KeyCode::Esc => {
                self.mode = InputMode::Normal;
            }
            _ => {}
        }
    }
    // ----------

    pub fn get_event_of_key(&mut self, key: KeyCode) -> Option<&Actions> {
        let event = self.keymap.get_command(key);

        // Manage the 'keys_queue' based in event received
        if let Some(Actions::SubCommand) = event {
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
