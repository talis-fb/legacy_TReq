use crate::base::actions::{manager::ActionsManager, Actions};
use crate::base::commands::{handler::CommandHandler, Command, Commands};
use crate::states::{default::DefaultState, State};
use crossterm::event::KeyCode;
use std::collections::hash_map::HashMap;

use tui::layout::Rect;
use tui::widgets::Widget;

use crate::base::web::request::{Request, METHODS};
use crate::input::listener::KeyboardListerner;

use crate::input::input::InputKeyboardBuffer;

use super::states::empty::EmptyState;
use super::states::manager::StateManager;

#[derive(Clone)]
pub enum InputMode {
    Normal,
    Insert,
}

pub struct App<'a> {
    pub current_request: usize,
    request_history: Vec<Request>,

    pub keymap: KeyboardListerner<'a>,
    pub log: String,
    keys_queue: String,
    mode: InputMode,
    input_buffer: InputKeyboardBuffer,

    // States
    pub state_manager: StateManager,

    // Actions
    pub action_manager: ActionsManager,

    // Commands
    pub command_handler: CommandHandler,
}

impl<'a> App<'a> {
    pub fn init(
        keymap: KeyboardListerner<'a>,
        state_manager: StateManager,
        action_manager: ActionsManager,
        command_handler: CommandHandler
    ) -> Self {
        // let state_manager = StateManager::init(DefaultState::init(), EmptyState::init());
        // let action_manager = ActionsManager {
        //     state_manager: &state_manager,
        // };
        // let command_handler = CommandHandler {};

        Self {
            current_request: 0,
            request_history: vec![],

            keymap,
            log: "".to_string(),
            keys_queue: "".to_string(),
            mode: InputMode::Normal,
            input_buffer: InputKeyboardBuffer::init(),

            state_manager,
            action_manager,
            command_handler,
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
