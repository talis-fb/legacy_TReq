#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::error::Error;

use treq::base::actions::manager::ActionsManager;
use treq::base::actions::Actions;
use treq::base::commands::handler::CommandHandler;
use treq::base::commands::{Command, Commands};

use treq::base::os::file_facades::variables::VariablesFile;
use treq::base::os::file_facades::FileFacade;
use treq::base::os::file_factory::{FileDefaultFactory, FileFactory};
use treq::base::os::handler::FileHandler;
use treq::base::os::os_commands::external_editor::ExternalEditor;
use treq::base::os::os_commands::{OsCommand, OsCommandTrait};
use treq::base::stores::MainStore;
use treq::base::web::client::WebClient;
use treq::base::web::repository::reqwest::ReqwestClientRepository;
use treq::base::web::request::Request;
use treq::config::configurations::view::ViewConfig;
use treq::config::manager::ConfigManager;

use treq::base::states::manager::StateManager;
use treq::base::states::states::{DefaultEditMode, DefaultHelpMode, DefaultState, State};
use treq::runner::Runner;
use treq::utils::custom_types::uuid::UUID;

// use std::sync::mpsc;
use tokio::sync::mpsc;

use treq::app::{App, InputMode};

use treq::input::keymaps;
use treq::input::listener::KeyboardListerner;

use treq::view::ui::UI;

use treq::input::input_handler::InputHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    treq::logger::init_logger();

    // ---------------------------------------------
    // Configurations and Setup of necessary folders
    // ---------------------------------------------
    FileHandler::setup_env_folder()
        .expect("Error creating folders of data at '.local/share/treq.'");
    let mut file_handler = FileHandler::default();
    file_handler.set_file_factory(FileDefaultFactory::default());

    FileDefaultFactory
        .get_saved_files_request()
        .unwrap()
        .into_iter()
        .for_each(|file| {
            file_handler.add_request(file);
        });

    if file_handler.get_map_files_request().is_empty() {
        let default_file = FileDefaultFactory
            .create_request_file(UUID::new(), Request::default())
            .unwrap();
        file_handler.add_request(default_file);
    }

    let path_saved_file_variables = VariablesFile::get_root_path().join("global_variables.json");
    let file_variables = FileDefaultFactory
        .get_saved_variables_file(path_saved_file_variables)
        .unwrap_or_else(|_| {
            FileDefaultFactory
                .create_variables_file("global_variables.json".to_string(), HashMap::new())
                .unwrap()
        });

    file_handler.add_variables(file_variables);

    // other configs
    let view_config = ViewConfig::init();

    // let external_editor = Box::new(ExternalEditor::init().unwrap());

    let config_manager = ConfigManager::init(file_handler, view_config);

    // Init of Data Stores
    let mut data_store = MainStore::init(config_manager);
    data_store.set_log_warning(String::from("NEEDING HELP,"), String::from("press [?]"));

    // Init Web Client
    let web_client: WebClient = WebClient::init(ReqwestClientRepository::default());

    // ------------------------------------------
    // Input
    // ------------------------------------------
    // EVENTS of actions
    // TODO: Remove this 32
    // TODO: Remove this 32
    // TODO: Remove this 32
    let (action_queue_sender, mut action_queue_receiver) = mpsc::channel::<Actions>(32);
    let (commands_queue_sender, mut commands_queue_receiver) = mpsc::channel::<Command>(32);
    let (os_commands_queue_sender, mut os_commands_queue_receiver) = mpsc::channel::<OsCommand>(32);

    let commands = keymaps::normal_mode::keymap_factory();
    let keymap = KeyboardListerner::init(commands);

    let mut input_handler = InputHandler::init(
        keymap,
        // data_store.config.editor.clone(),
        data_store.config.files.clone(),
        action_queue_sender.clone(),
    );

    // ------------------------------------------
    // View
    // ------------------------------------------
    let view = UI::init();

    // ------------------------------------------
    // Init app -> it starts with a empty request
    // ------------------------------------------
    let state_manager = StateManager::init(DefaultState::init(), DefaultState::init());
    let action_manager = ActionsManager::init();

    let mut app = App::default();
    app.set_state_manager(state_manager);
    app.set_action_manager(action_manager);
    app.set_web_client(web_client);
    app.set_data_store(data_store);
    app.set_renderer(action_queue_sender);

    let command_handler = CommandHandler::init(commands_queue_sender.clone());

    let mut runner = Runner::init(app, command_handler, input_handler, view);

    runner.set_receiver_actions_queue(action_queue_receiver);
    runner.set_receiver_commands_queue(commands_queue_receiver);
    runner.set_receiver_os_commands_queue(os_commands_queue_receiver);

    while !runner.is_finished() {
        runner.render();
        runner.update_input_handler();
        runner.proccess().await;
    }
    
    runner.close();


    // while !app.is_finished {
    //     view.render(app.get_data_store());
    //
    //     input_handler.update(app.get_mode());
    //
    //     match app.get_mode() {
    //         InputMode::Help => {
    //             app.set_new_state(DefaultHelpMode::init());
    //         }
    //
    //         InputMode::Insert => {
    //             app.set_new_state(DefaultEditMode::init());
    //         }
    //
    //         _ => {}
    //     }
    //
    //     tokio::select! {
    //         action = action_queue_receiver.recv() => {
    //             log::info!("Action {:?}", action);
    //
    //             let command = app
    //                 .get_command_of_action(action.unwrap())
    //                 .unwrap_or(Commands::do_nothing());
    //
    //             // Add Command to queue
    //             command_handler.add(command);
    //         }
    //         command = commands_queue_receiver.recv() => {
    //             let command_result = command_handler.run(command.unwrap(), &mut app);
    //
    //             if let Err(e) = command_result {
    //                 app.get_data_store_mut()
    //                     .set_log_error(String::from("COMMAND ERROR"), e.to_string())
    //             }
    //         }
    //         os_command = os_commands_queue_receiver.recv() => {
    //             match os_command.unwrap() {
    //                 OsCommand::Sync(comm) => {
    //                     view.close();
    //
    //                     let output = comm.exec(commands_queue_sender.clone());
    //
    //                     if let Err(e) = output {
    //                         app.get_data_store_mut()
    //                             .set_log_error(String::from("OS COMMAND ERROR"), e.to_string())
    //                     }
    //
    //                     view = UI::init();
    //                     view.render(app.get_data_store());
    //
    //                     while action_queue_receiver.try_recv().is_ok() {
    //                         log::info!("Clear queue");
    //                     }
    //                 }
    //
    //                 OsCommand::Async(comm) => {
    //                     let sender = commands_queue_sender.clone();
    //                     tokio::task::spawn(async move {
    //                         comm.exec(sender).unwrap();
    //                     });
    //                 }
    //             }
    //         }
    //         
    //     }
    //
    //
    //     // while let Ok(command) = os_commands_queue_receiver.try_recv() {
    //     // }
    //
    //
    //     // Listen queue of user's events to execute --------------------
    //     // log::info!("Wainting action....");
    //     // match action_queue_receiver.recv() {
    //     //     Ok(action_to_exec) => {
    //     //
    //     //         // exec it
    //     //     }
    //     //     Err(err) => {
    //     //         log::error!("Action ERROR");
    //     //         log::error!("{}", err);
    //     //     }
    //     // }
    // }

    Ok(())
}
