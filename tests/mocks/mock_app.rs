use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Mutex;
use tokio::sync::mpsc::{self, Receiver, Sender};
use treq::input::input_handler::MockInputHandler;

use std::collections::HashMap;

use crate::mocks::file_factory::MockFileFactory;
use treq::app::{App, InputMode};
use treq::base::actions::manager::ActionsManager;
use treq::base::commands::handler::CommandHandler;
use treq::base::commands::{Command, Commands};
// use treq::base::os::file_editor::{MockOsCommand, OsCommandEditor};
use treq::base::os::os_commands::{MockOsCommandTrait, OsCommand};
use treq::base::states::manager::StateManager;
use treq::base::states::states::{DefaultEditMode, DefaultHelpMode, DefaultState, State};
use treq::base::web::client::WebClient;
use treq::base::web::repository::MockHttpClientRepository;
use treq::runner::Runner;
use treq::view::MockUiTrait;
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

#[cfg(test)]
pub struct MockApp {
    pub runner: Runner<MockUiTrait, MockInputHandler>,
    pub queue_actions_sender: Sender<Actions>,
    pub queue_commands_sender: Sender<Command>,
    pub queue_os_commands_sender: Sender<OsCommand>,

    pub history_commands: Vec<Result<(), String>>,
    // Input Mock
    // pub buffer_input: String,
    // pub opened_files: HashMap<UUID, UUID>,
}

#[cfg(test)]
impl MockApp {
    pub fn init() -> Self {
        let mut file_handler = FileHandler::default();
        file_handler.set_file_factory(MockFileFactory::default());

        let default_request_file = file_handler
            .file_factory
            .as_mut()
            .unwrap()
            .create_request_file(UUID::new(), Request::default())
            .unwrap();
        file_handler.add_request(default_request_file);

        let default_variables_file = file_handler
            .file_factory
            .as_mut()
            .unwrap()
            .create_variables_file("global_variables.json".to_string(), HashMap::new())
            .unwrap();
        file_handler.add_variables(default_variables_file);

        let view_config = ViewConfig::init();
        let external_editor = MockOsCommandTrait::new();

        let config_manager = ConfigManager::init(file_handler, view_config);

        let data_store = MainStore::init(config_manager);

        let web_client: WebClient = WebClient::init(MockHttpClientRepository::default());

        let (queue_actions_sender, queue_actions_receiver) = mpsc::channel::<Actions>(64);
        let (queue_commands_sender, queue_commands_receiver) = mpsc::channel::<Command>(64);
        let (queue_os_commands_sender, queue_os_commands_receiver) = mpsc::channel::<OsCommand>(64);

        let state_manager = StateManager::init(DefaultState::init(), DefaultState::init());
        let action_manager = ActionsManager::init();

        let mut app = App::default();
        app.set_state_manager(state_manager);
        app.set_action_manager(action_manager);
        app.set_web_client(web_client);
        app.set_data_store(data_store);
        app.set_renderer(queue_actions_sender.clone());

        let command_handler = CommandHandler::init(queue_commands_sender.clone());


        let mut view = MockUiTrait::new();
        view.expect_render().return_const(());
        view.expect_restart().return_const(());
        view.expect_close().return_const(());

        let mut input_handler = MockInputHandler::new();
        input_handler.expect_update().return_const(());

        let mut runner = Runner::init(app, command_handler, input_handler, view);

        runner.set_receiver_actions_queue(queue_actions_receiver);
        runner.set_receiver_commands_queue(queue_commands_receiver);
        runner.set_receiver_os_commands_queue(queue_os_commands_receiver);

        Self {
            runner,
            history_commands: vec![],
            queue_commands_sender,
            queue_os_commands_sender,
            queue_actions_sender
        }
    }

    pub fn is_finished(&self) -> bool {
        self.runner.app.is_finished
    }

    pub async fn exec(&mut self, action: Actions) {
        self.push_action(action).await;
        self.run_commands_in_queue().await;
    }

    pub async fn push_action(&mut self, action: Actions) {
        self.queue_actions_sender.send(action).await.unwrap();
    }

    pub async fn run_commands_in_queue(&mut self) {
        self.runner.render();
        self.runner.update_input_handler();
        self.runner.proccess().await;

        // match self.app.get_mode() {
        //     InputMode::Vim => {
        //         let buf = self.app.get_input_buffer_value();
        //         let uuid = self.app.get_data_store().get_request_uuid().clone();
        //
        //         let buffer = self.sync_open_vim(buf, &uuid).unwrap();
        //
        //         // Set Buffer and return to Normal Mode
        //         self.app.set_input_buffer_value(buffer);
        //         self.app.exec_input_buffer_command().unwrap();
        //         self.app.set_mode(InputMode::Normal);
        //     }
        //
        //     InputMode::Help => {
        //         self.app.set_new_state(DefaultHelpMode::init());
        //     }
        //
        //     InputMode::Insert => {
        //         self.app.set_new_state(DefaultEditMode::init());
        //     }
        //
        //     _ => {}
        // }
        //
        // while let Ok(action_to_exec) = self.queue_actions_receiver.try_recv() {
        //     log::info!("Action {:?}", action_to_exec);
        //
        //     let command = self
        //         .app
        //         .get_command_of_action(action_to_exec)
        //         .unwrap_or(Commands::do_nothing());
        //
        //     // Add Command to queue
        //     self.command_handler.add(command);
        //
        //     // exec it
        //     let command_result = self.command_handler.run(&mut self.app);
        //
        //     if let Err(e) = &command_result {
        //         self.app
        //             .get_data_store_mut()
        //             .set_log_error(String::from("COMMAND ERROR"), e.to_string())
        //     }
        //
        //     self.history_commands.push(command_result);
        // }
    }

    // fn sync_open_vim(&mut self, buffer: String, uuid_edition: &UUID) -> Result<String, String> {
    //     let mut files = self.app.get_data_store().config.files.lock().unwrap();
    //
    //     let uuid_file_handler = self
    //         .opened_files
    //         .entry(uuid_edition.clone())
    //         .or_insert_with(|| {
    //             let file_to_add = files
    //                 .file_factory
    //                 .as_mut()
    //                 .unwrap()
    //                 .create_temp_file(uuid_edition.clone(), buffer)
    //                 .unwrap();
    //             files.add_temp_edition(file_to_add)
    //         });
    //
    //     let file_path = files.get_path(uuid_file_handler).unwrap();
    //
    //     self.app.get_data_store().config.editor.sync_open(file_path)
    // }

    // pub fn set_output_sync_external_editor(&mut self, output: String) {
    //     let mut mock = MockOsCommandTrait::default();
    //     mock.expect_sync_open().return_const(Ok(output));
    //     self.app.get_data_store_mut().config.editor = Rc::new(Box::new(mock));
    // }

    // pub fn set_external_editor(&mut self, editor: MockOsCommandTrait<PathBuf, String>) {
    //     self.app.get_data_store_mut().config.editor = Rc::new(Box::new(editor));
    // }

}
