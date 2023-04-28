use std::sync::mpsc::{self, Receiver, Sender};

use std::collections::HashMap;
use std::time::Duration;

use crate::mocks::file_factory::MockFileFactory;
use treq::app::{App, InputMode};
use treq::base::actions::manager::ActionsManager;
use treq::base::commands::handler::CommandHandler;
use treq::base::commands::Commands;
use treq::base::states::manager::StateManager;
use treq::base::states::states::{DefaultEditMode, DefaultHelpMode, DefaultState, State};
use treq::base::web::client::WebClient;
use treq::base::web::repository::{HttpClientRepository, MockHttpClientRepository};
use treq::{
    base::{
        actions::Actions,
        os::{file_factory::FileFactory, handler::FileHandler},
        stores::MainStore,
        web::request::Request,
    },
    config::{
        configurations::{external_editor::ExternalEditor, view::ViewConfig},
        manager::ConfigManager,
    },
    utils::custom_types::uuid::UUID,
};

pub struct MockApp {
    pub app: App,
    pub command_handler: CommandHandler,
    pub queue_actions_sender: Sender<Actions>,
    pub queue_actions_receiver: Receiver<Actions>,

    pub history_commands: Vec<Result<(), String>>,

    // Input Mock
    pub buffer_input: String,
}

impl MockApp {
    pub fn init() -> Self {
        let mut file_handler = FileHandler::default();
        file_handler.set_file_factory(Box::<MockFileFactory>::default());

        let default_request_file = MockFileFactory
            .create_request_file(UUID::new(), Request::default())
            .unwrap();
        file_handler.add_request(default_request_file);

        let default_variables_file = MockFileFactory
            .create_variables_file("global_variables.json".to_string(), HashMap::new())
            .unwrap();
        file_handler.add_variables(default_variables_file);

        let view_config = ViewConfig::init();
        let external_editor = ExternalEditor::setup_and_init().expect(
            "It's necessary set $EDITOR enviroment variable to desired editor to use with TReq",
        );

        let config_manager = ConfigManager::init(file_handler, view_config, external_editor);

        let data_store = MainStore::init(config_manager);

        let web_client: WebClient = WebClient::init(MockHttpClientRepository::default());

        let (queue_actions_sender, queue_actions_receiver): (Sender<Actions>, Receiver<Actions>) =
            mpsc::channel();

        let state_manager = StateManager::init(DefaultState::init(), DefaultState::init());
        let action_manager = ActionsManager::init();

        let mut app = App::default();
        app.set_state_manager(state_manager);
        app.set_action_manager(action_manager);
        app.set_web_client(web_client);
        app.set_data_store(data_store);
        app.set_renderer(queue_actions_sender.clone());

        let command_handler = CommandHandler::init();

        Self {
            app,
            command_handler,
            queue_actions_receiver,
            queue_actions_sender,
            history_commands: Vec::new(),
            buffer_input: String::new(),
        }
    }

    pub fn exec(&mut self, action: Actions) {
        self.push_action(action);
        self.run_commands_in_queue();
    }

    pub fn push_action(&mut self, action: Actions) {
        self.queue_actions_sender.send(action).unwrap();
    }

    pub fn run_commands_in_queue(&mut self) {
        match self.app.get_mode() {
            InputMode::Help => {
                self.app.set_new_state(DefaultHelpMode::init());
            }

            InputMode::Insert => {
                self.app.set_new_state(DefaultEditMode::init());
            }

            _ => {}
        }

        while let Ok(action_to_exec) = self.queue_actions_receiver.try_recv() {
            log::info!("Action {:?}", action_to_exec);

            // panic!("Entrou nessa porra");

            let command = self
                .app
                .get_command_of_action(action_to_exec)
                .unwrap_or(Commands::do_nothing());

            // Add Command to queue
            self.command_handler.add(command);

            // exec it
            let command_result = self.command_handler.run(&mut self.app);

            if let Err(e) = &command_result {
                self.app
                    .get_data_store_mut()
                    .set_log_error(String::from("COMMAND ERROR"), e.to_string())
            }

            self.history_commands.push(command_result);
        }
    }

    pub fn set_return_input<F>(&self, func: F) -> String
    where
        F: Fn(String) -> String,
    {
        func(self.buffer_input.clone())
    }

    pub fn is_finished(&self) -> bool {
        self.app.is_finished
    }
}

// Functions
// -> Define a MockInputEditor, where you can define a closure which receive
//      value of buffer and return what "user" would change on it. This is
//      called when InputMode is "Vim"
//

// pub fn hello() {
//     while !app.is_finished {
//         match app.get_mode() {
//             InputMode::Help => {
//                 app.set_new_state(DefaultHelpMode::init());
//             }
//
//             InputMode::Insert => {
//                 app.set_new_state(DefaultEditMode::init());
//             }
//
//             _ => {}
//         }
//
//         // Listen queue of user's events to execute --------------------
//         log::info!("Wainting action....");
//         match action_queue_receiver.recv() {
//             Ok(action_to_exec) => {
//                 log::info!("Action {:?}", action_to_exec);
//
//                 let command = app
//                     .get_command_of_action(action_to_exec)
//                     .unwrap_or(Commands::do_nothing());
//
//                 // Add Command to queue
//                 command_handler.add(command);
//
//                 // exec it
//                 let command_result = command_handler.run(&mut app);
//
//                 if let Err(e) = command_result {
//                     app.get_data_store_mut()
//                         .set_log_error(String::from("COMMAND ERROR"), e.to_string())
//                 }
//             }
//             Err(err) => {
//                 log::error!("Action ERROR");
//                 log::error!("{}", err);
//             }
//         }
//     }
// }
