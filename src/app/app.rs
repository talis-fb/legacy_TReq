use crate::base::actions::{manager::ActionsManager, Actions};
use crate::base::commands::{handler::CommandHandler, Command, Commands};
use crate::base::web::client::WebClient;
use crate::base::web::repository::HttpClientRepository;
use crate::base::web::repository::reqwest::ReqwestClientRepository;
use crate::states::{default::DefaultState, State};
use crossterm::event::KeyCode;
use std::collections::hash_map::HashMap;
use std::sync::Arc;
use std::thread;

use tui::layout::Rect;
use tui::widgets::Widget;

use crate::base::web::request::{Request, METHODS};
use crate::input::listener::KeyboardListerner;

use crate::input::input::InputKeyboardBuffer;

use super::states::empty::EmptyState;
use super::states::manager::StateManager;
use super::states::States;

#[derive(Clone)]
pub enum InputMode {
    Normal,
    Insert,
}

pub struct App<'a> {
    pub current_request: usize,
    request_history: Vec<Request>,

    pub log: String,
    keys_queue: String,
    mode: InputMode,
    input_buffer: InputKeyboardBuffer,

    // KeyboardListerner
    pub keymap: Option<KeyboardListerner<'a>>,

    // States
    pub state_manager: Option<StateManager>,

    // Actions
    pub action_manager: Option<ActionsManager>,

    // Commands
    pub command_handler: Option<CommandHandler>,

    // Web Client
    pub client_web: Option<Arc<WebClient<ReqwestClientRepository>>>
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            current_request: 0,
            request_history: vec![],
            log: String::from(""),
            keys_queue: String::from(""),
            mode: InputMode::Normal,
            input_buffer: InputKeyboardBuffer::init(),

            keymap: None,
            state_manager: None,
            action_manager: None,
            command_handler: None,
            client_web: None
        }
    }
}

impl<'a> App<'a> {
    // Builders --------
    pub fn set_keymap(&mut self, keymap: KeyboardListerner<'a>) -> () {
        self.keymap = Some(keymap)
    }
    pub fn set_state_manager(&mut self, state_manager: StateManager) -> () {
        self.state_manager = Some(state_manager)
    }
    pub fn set_action_manager(&mut self, action_manager: ActionsManager) -> () {
        self.action_manager = Some(action_manager)
    }
    pub fn set_command_handler(&mut self, command_handler: CommandHandler) -> () {
        self.command_handler = Some(command_handler)
    }

    // Manage States
    pub fn set_new_state(&mut self, new_state: impl State + 'static) -> Option<()> {
        self.state_manager.as_mut()?.set_state(new_state);
        Some(())
    }
    pub fn get_state(&self, new_state: impl State + 'static) -> Option<&Box<dyn State>> {
        Some(self.state_manager.as_ref()?.get_state())
    }

    // Commands
    pub fn get_command_of_action(&self, action: Actions) -> Option<Command> {
        let state_manager = self.state_manager.as_ref()?;
        self.action_manager
            .as_ref()?
            .get_command_of_action(action, &state_manager)
    }


    // Web client
    pub fn set_web_client(&mut self, client: WebClient<ReqwestClientRepository>) -> () {
        self.client_web = Some(Arc::new(client))
    }

    pub fn submit(&self) -> () {
        let request = self.get_current_request().clone();
        let client = self.client_web.as_ref().unwrap().clone();

        tokio::task::spawn(async move {
            let response = client.submit(request).await;
        });
    }


    // Input Mode ------
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
        let event = self.keymap.as_mut()?.get_command(key);

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