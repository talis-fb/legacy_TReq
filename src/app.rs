use crate::base::actions::{manager::ActionsManager, Actions};
use crate::base::commands::{handler::CommandHandler, Command};
use crate::base::states::manager::StateManager;
use crate::base::states::states::State;
use crate::base::stores::MainStore;
use crate::base::web::client::WebClient;
use crate::base::web::repository::reqwest::ReqwestClientRepository;
use crate::base::web::response::Response;
use crate::config::configurations::save_files::SaveFiles;
use crate::input::buffer::InputKeyboardBuffer;
use std::sync::mpsc::Sender;
use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
pub enum InputMode {
    Normal,
    Insert,
    Vim,
    Help,
}

#[derive(Default)]
pub struct App {
    pub is_finished: bool,
    renderer: Option<Sender<Actions>>,

    // Datas
    pub data_store: Option<MainStore>,
    pub save_files: Option<SaveFiles>,

    // States
    pub state_manager: Option<StateManager>,

    // Actions
    pub action_manager: Option<ActionsManager>,

    // Web Client
    pub client_web: Option<Arc<WebClient<ReqwestClientRepository>>>,
}

impl App {
    // Builders -------- ---------------------
    pub fn set_data_store(&mut self, data_store: MainStore) {
        self.data_store = Some(data_store)
    }
    pub fn set_save_file(&mut self, save_files: SaveFiles) {
        self.save_files = Some(save_files)
    }
    pub fn set_state_manager(&mut self, state_manager: StateManager) {
        self.state_manager = Some(state_manager)
    }
    pub fn set_action_manager(&mut self, action_manager: ActionsManager) {
        self.action_manager = Some(action_manager)
    }
    pub fn set_web_client(&mut self, client: WebClient<ReqwestClientRepository>) {
        self.client_web = Some(Arc::new(client))
    }
    pub fn set_renderer(&mut self, renderer: Sender<Actions>) {
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
        self.set_input_buffer_value(initial_buffer);
    }
    pub fn set_vim_mode_with_command(&mut self, callback: Command, initial_buffer: String) {
        self.set_mode(InputMode::Vim);
        let data_store = self.get_data_store_mut();
        data_store.input_buffer.command = callback;

        self.get_input_buffer_mut()
            .set_backup(initial_buffer.clone());
        self.set_input_buffer_value(initial_buffer);
    }
    pub fn get_input_buffer(&mut self) -> &InputKeyboardBuffer {
        &self.get_data_store_mut().input_buffer
    }
    pub fn get_input_buffer_mut(&mut self) -> &mut InputKeyboardBuffer {
        &mut self.get_data_store_mut().input_buffer
    }
    pub fn get_input_buffer_value(&self) -> String {
        self.get_data_store().input_buffer.value.clone()
    }
    pub fn set_input_buffer_value(&mut self, buffer: String) {
        self.get_data_store_mut().input_buffer.value = buffer;
    }
    pub fn exec_input_buffer_command(&mut self) -> Result<(), String> {
        let command_fn = self.get_data_store_mut().input_buffer.command.clone();
        command_fn.execute(self)
    }

    // Manage States ---------------------------
    pub fn get_state(&self) -> Option<&Box<dyn State>> {
        Some(self.state_manager.as_ref()?.get_state())
    }
    pub fn set_new_state(&mut self, new_state: impl State + 'static) -> Option<()> {
        self.get_data_store_mut().current_state = new_state.get_state_name();
        self.state_manager.as_mut()?.set_state(new_state);
        Some(())
    }

    pub fn reset_to_last_state(&mut self) -> Option<()> {
        let state = self.state_manager.as_mut()?.reset_to_last_state();
        self.get_data_store_mut().current_state = state;
        Some(())
    }

    // Commands ---------------------
    pub fn get_command_of_action(&mut self, action: Actions) -> Option<Command> {
        let state_manager = self.state_manager.as_ref()?;
        self.action_manager
            .as_mut()?
            .get_command_of_action(action, state_manager)
    }

    // Web client ---------------------
    pub fn dispatch_submit(&self) {
        let client = self.client_web.as_ref().unwrap().clone();
        let request = self.data_store.as_ref().unwrap().get_request();
        let response_data_store = self.data_store.as_ref().unwrap().get_response();

        let data_store = self.get_data_store().clone();

        let renderer = self.renderer.as_ref().unwrap().clone();

        tokio::task::spawn(async move {
            let new_response = client.submit((*request).clone()).await;

            let mut data = response_data_store.lock().unwrap();

            *data = new_response.unwrap_or_else(Response::default_internal_error);
            renderer.send(Actions::Null).unwrap();
        });
    }

    // Data store ---------------------
    pub fn get_data_store(&self) -> &MainStore {
        self.data_store.as_ref().unwrap()
    }

    pub fn get_data_store_mut(&mut self) -> &mut MainStore {
        self.data_store.as_mut().unwrap()
    }

    pub fn clear_log(&mut self) {
        self.get_data_store_mut().clear_log()
    }

    pub fn rerender(&self) {
        self.renderer.as_ref().unwrap().send(Actions::Null).unwrap();
    }
}
