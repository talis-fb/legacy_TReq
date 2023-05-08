#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::error::Error;

use clap::Parser;
use treq::base::actions::manager::ActionsManager;
use treq::base::actions::Actions;
use treq::base::commands::handler::CommandHandler;
use treq::base::commands::Command;

use treq::base::os::file_facades::variables::VariablesFile;
use treq::base::os::file_facades::FileFacade;
use treq::base::os::file_factory::{FileDefaultFactory, FileFactory};
use treq::base::os::handler::FileHandler;

use treq::base::os::os_commands::external_editor::ExternalEditor;
use treq::base::os::os_commands::factory::OsCommandDefaultFactory;
use treq::base::os::os_commands::OsCommand;
use treq::base::stores::MainStore;
use treq::base::web::client::WebClient;
use treq::base::web::repository::reqwest::ReqwestClientRepository;
use treq::base::web::request::Request;
use treq::cli::Args;
use treq::config::configurations::view::ViewConfig;
use treq::config::manager::ConfigManager;

use treq::base::states::manager::StateManager;
use treq::base::states::states::{DefaultState, State};
use treq::runner::Runner;
use treq::utils::custom_types::uuid::UUID;

use tokio::sync::mpsc;

use treq::app::App;

use treq::input::keymaps;
use treq::input::listener::KeyboardListerner;

use treq::view::ui::UI;

use treq::input::input_handler::InputDefaultHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    #[cfg(debug_assertions)]
    treq::logger::init_logger();

    // Setup CLI options
    Args::parse();

    if !ExternalEditor::is_valid() {
        println!();
        println!("-------------------   ERROR: Undefined EDITOR or TREQ_EDITOR   ---------------------- ");
        println!(" TREQ requires EDITOR or TREQ_EDITOR env variables to setup an external editor");
        println!();
        println!(
            " For a guide how to do this quickly: https://github.com/talis-fb/TReq/wiki/Editor"
        );
        println!("-------------------------------------------------------------------------------------- ");
        println!();
        panic!()
    }

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

    let config_manager = ConfigManager::init(file_handler, view_config);

    // Init of Data Stores
    let mut data_store = MainStore::init(config_manager);
    data_store.set_log_warning(String::from("NEEDING HELP,"), String::from("press [?]"));

    // Init Web Client
    let web_client: WebClient = WebClient::init(ReqwestClientRepository::default());

    // ------------------------------------------
    // Input
    // ------------------------------------------
    let (action_queue_sender, action_queue_receiver) = mpsc::channel::<Actions>(32);
    let (commands_queue_sender, commands_queue_receiver) = mpsc::channel::<Command>(32);
    let (os_commands_queue_sender, os_commands_queue_receiver) = mpsc::channel::<OsCommand>(32);

    let commands = keymaps::normal_mode::keymap_factory();
    let keymap = KeyboardListerner::init(commands);

    let input_handler = InputDefaultHandler::init(
        keymap,
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
    app.set_os_commands_queue(os_commands_queue_sender.clone());
    app.set_os_commands_factory(OsCommandDefaultFactory {});

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

    Ok(())
}
