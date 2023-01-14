use crate::base::actions::{manager::ActionsManager, Actions};
use crate::base::commands::{handler::CommandHandler, Command, Commands};
use crate::base::logs::{Log, LogType};
use crate::base::web::client::WebClient;
use crate::base::web::repository::reqwest::ReqwestClientRepository;
use crate::base::web::repository::HttpClientRepository;
use crate::base::web::response::Response;
use crate::config::saves::SaveFiles;
use crate::states::{default::DefaultState, State};
use crossterm::event::KeyCode;
use std::collections::hash_map::HashMap;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::thread;

use tui::layout::Rect;
use tui::widgets::Widget;

use crate::base::web::request::{Request, METHODS};
use crate::input::listener::KeyboardListerner;

use crate::input::buffer::{InputBuffer, InputKeyboardBuffer};

use super::states::empty::EmptyState;
use super::states::manager::StateManager;
use super::states::States;

use crate::base::store::MainStore;

#[derive(Copy, Clone, Debug)]
pub enum InputMode {
    Normal,
    Insert,
    Vim,
    Help,
}

pub struct App {
    pub is_finished: bool,
    renderer: Option<Sender<MainStore>>,

    // Datas
    pub data_store: Option<MainStore>,
    pub save_files: Option<SaveFiles>,

    // States
    pub state_manager: Option<StateManager>,

    // Actions
    pub action_manager: Option<ActionsManager>,

    // Commands
    pub command_handler: Option<CommandHandler>,

    // Web Client
    pub client_web: Option<Arc<WebClient<ReqwestClientRepository>>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            is_finished: false,

            renderer: None,
            data_store: None,
            save_files: None,
            state_manager: None,
            action_manager: None,
            command_handler: None,
            client_web: None,
        }
    }
}

impl App {
    // Builders -------- ---------------------
    pub fn set_data_store(&mut self, data_store: MainStore) -> () {
        self.data_store = Some(data_store)
    }
    pub fn set_save_file(&mut self, save_files: SaveFiles) -> () {
        self.save_files = Some(save_files)
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
    pub fn set_web_client(&mut self, client: WebClient<ReqwestClientRepository>) -> () {
        self.client_web = Some(Arc::new(client))
    }
    pub fn set_renderer(&mut self, renderer: Sender<MainStore>) -> () {
        self.renderer = Some(renderer)
    }

    // Modes & Input ---------------------------
    pub fn get_mode(&self) -> InputMode {
        self.get_data_store().mode
    }
    pub fn set_mode(&mut self, mode: InputMode) {
        self.get_data_store_mut().mode = mode;
    }
    pub fn set_input_mode_with_command(&mut self, callback: Command, initial_buffer: String) {
        self.set_mode(InputMode::Insert);
        let data_store = self.get_data_store_mut();

        data_store.input_buffer.command = callback;
        data_store.set_log_input_mode();
        self.get_input_buffer_mut()
            .set_backup(initial_buffer.clone());
        self.set_input_buffer_value(initial_buffer.clone());
    }
    pub fn set_vim_mode_with_command(&mut self, callback: Command, initial_buffer: String) {
        self.set_mode(InputMode::Vim);
        let data_store = self.get_data_store_mut();
        data_store.input_buffer.command = callback;

        self.get_input_buffer_mut()
            .set_backup(initial_buffer.clone());
        self.set_input_buffer_value(initial_buffer.clone());
    }
    pub fn get_input_buffer(&mut self) -> &InputKeyboardBuffer {
        &self.get_data_store_mut().input_buffer
    }
    pub fn get_input_buffer_mut(&mut self) -> &mut InputKeyboardBuffer {
        &mut self.get_data_store_mut().input_buffer
    }
    pub fn get_input_buffer_value(&mut self) -> String {
        self.get_data_store_mut().input_buffer.value.clone()
    }
    pub fn set_input_buffer_value(&mut self, buffer: String) {
        self.get_data_store_mut().input_buffer.value = buffer;
    }
    pub fn exec_input_buffer_command(&mut self) {
        let command_fn = self.get_data_store_mut().input_buffer.command;
        command_fn(self);
    }

    // Manage States ---------------------------
    pub fn get_state(&self) -> Option<&Box<dyn State>> {
        Some(self.state_manager.as_ref()?.get_state())
    }
    pub fn set_new_state(&mut self, new_state: impl State + 'static) -> Option<()> {
        self.get_data_store_mut().current_state = new_state.get_state_name().clone();
        self.state_manager.as_mut()?.set_state(new_state);
        Some(())
    }

    // Commands ---------------------
    pub fn get_command_of_action(&self, action: Actions) -> Option<Command> {
        let state_manager = self.state_manager.as_ref()?;
        self.action_manager
            .as_ref()?
            .get_command_of_action(action, &state_manager)
    }

    // Web client ---------------------
    pub fn dispatch_submit(&self) -> () {
        let client = self.client_web.as_ref().unwrap().clone();
        let request = self.data_store.as_ref().unwrap().get_request().clone();
        let response_data_store = self.data_store.as_ref().unwrap().get_response().clone();

        let data_store = self.get_data_store().clone();

        tokio::task::spawn(async move {
            let new_response = client
                .submit((*request).clone())
                .await
                .map_err(|e| e.to_string());

            let mut data = response_data_store.lock().unwrap();

            *data = new_response.unwrap_or_else(|err| Response::default_internal_error(err))
        });
    }

    // Data store ---------------------
    pub fn get_data_store(&self) -> &MainStore {
        self.data_store.as_ref().unwrap()
    }

    pub fn get_data_store_mut(&mut self) -> &mut MainStore {
        self.data_store.as_mut().unwrap()
    }

    pub fn clear_log(&mut self) -> () {
        self.get_data_store_mut().clear_log()
    }
}
