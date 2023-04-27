#![allow(dead_code)]
#![allow(unused_variables)]
use std::collections::HashMap;
use std::error::Error;

use treq::base::actions::manager::ActionsManager;
use treq::base::actions::Actions;
use treq::base::commands::handler::CommandHandler;
use treq::base::commands::Commands;
use treq::base::os::file_facades::variables::VariablesFile;
use treq::base::os::file_facades::FileFacade;

use treq::base::os::file_factory::{FileDefaultFactory, FileFactory};
use treq::base::os::handler::FileHandler;
use treq::base::stores::MainStore;
use treq::base::web::client::WebClient;
use treq::base::web::repository::reqwest::ReqwestClientRepository;
use treq::base::web::request::Request;
use treq::config::configurations::external_editor::ExternalEditor;
use treq::config::configurations::view::ViewConfig;
use treq::config::manager::ConfigManager;

use treq::base::states::manager::StateManager;
use treq::base::states::states::{DefaultEditMode, DefaultHelpMode, DefaultState, State};
use treq::utils::custom_types::uuid::UUID;

use std::sync::mpsc::{self, Receiver, Sender};

use treq::app::{App, InputMode};

use treq::input::keymaps;
use treq::input::listener::KeyboardListerner;

use treq::view::ui::UI;

use treq::input::input_handler::InputHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    treq::logger::init_logger();

    // 1- Chamar os setup_env do config manager aq no comeco da main()
    // 2- Passar o file_handler como injecao de dependencia pro ConfigManager
    //  (os mocks dos FileFacades serão usados pelo FileHandler na execução, passando para add)
    // 3-

    // No mock...
    // 1- Iniciar o FileHandler e MockWebClient
    // 2- Iniciar o ConfigManager( <-FileHandler)
    // 3- DataStore( <-ConfigManager )
    // 4- let (action_queue_sender, action_queue_receiver): (Sender<Actions>, Receiver<Actions>) = mpsc::channel();
    // 5- Init app e set builders

    // ---------------------------------------------
    // Configurations and Setup of necessary folders
    // ---------------------------------------------
    FileHandler::setup_env_folder()
        .expect("Error creating folders of data at '.local/share/treq.'");
    let mut file_handler = FileHandler::default();
    file_handler.set_file_factory(Box::<FileDefaultFactory>::default());

    FileDefaultFactory
        .get_saved_files_request()
        .unwrap()
        .into_iter()
        .for_each(|file| {
            file_handler.add_request(file);
        });

    if file_handler.get_map_files_request().is_empty() {
        let default_file = FileDefaultFactory.create_request_file(UUID::new(), Request::default()).unwrap();
        file_handler.add_request(default_file);
    }

    let file_variables = FileDefaultFactory.create_variables_file("global_variables.json".to_string(), HashMap::new()).unwrap();
    file_handler.add_variables(file_variables);

    // other configs
    let view_config = ViewConfig::init();
    let external_editor = ExternalEditor::setup_and_init().expect(
        "It's necessary set $EDITOR enviroment variable to desired editor to use with TReq",
    );

    let config_manager = ConfigManager::init(file_handler, view_config, external_editor);

    // Init of Data Stores
    let mut data_store = MainStore::init(config_manager);
    data_store.set_log_warning(String::from("NEEDING HELP,"), String::from("press [?]"));

    // Init Web Client
    let web_client: WebClient<ReqwestClientRepository> =
        WebClient::init(ReqwestClientRepository::default());

    // ------------------------------------------
    // Input
    // ------------------------------------------
    // EVENTS of actions
    let (action_queue_sender, action_queue_receiver): (Sender<Actions>, Receiver<Actions>) =
        mpsc::channel();

    let commands = keymaps::normal_mode::keymap_factory();
    let keymap = KeyboardListerner::init(commands);

    let mut input_handler = InputHandler::init(
        keymap,
        data_store.config.editor.clone(),
        data_store.config.files.clone(),
        action_queue_sender.clone(),
    );

    // File handler
    // Config(<<<)
    // MainStore(<<Config)
    // WebClient
    // Action queue definition
    // Init app
    //  Pass ever dependeny to App
    //  -> state_manager
    //  -> action_manager
    //  -> web_client
    //  -> MainStore
    //  -> Action queue sender
    //  Defintion CommandHandler

    // ------------------------------------------
    // View
    // ------------------------------------------
    let mut view = UI::init();

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

    let mut command_handler = CommandHandler::init();

    while !app.is_finished {
        view.render(app.get_data_store());

        match app.get_mode() {
            InputMode::Vim => {
                // Closes UI to dont conflit with external APP
                view.close();

                // Update and open it, getting result
                input_handler.update(InputMode::Vim);
                let buffer = input_handler
                    .sync_open_vim(
                        app.get_input_buffer_value(),
                        app.get_data_store().get_request_uuid(),
                    )
                    .unwrap();

                // Set Buffer and return to Normal Mode
                app.set_input_buffer_value(buffer);
                app.exec_input_buffer_command()?;

                app.set_mode(InputMode::Normal);
                input_handler.update(InputMode::Normal);

                // Restart UI, set Buffer and return to Normal Mode
                view = UI::init();
                view.render(app.get_data_store());

                // Clear queue if some event was catchig by event:read() in external editor
                while action_queue_receiver.try_recv().is_ok() {
                    log::info!("Clear queue");
                }
            }

            mode => {
                input_handler.update(mode);
            }
        }

        match app.get_mode() {
            InputMode::Help => {
                app.set_new_state(DefaultHelpMode::init());
            }

            InputMode::Insert => {
                app.set_new_state(DefaultEditMode::init());
            }

            _ => {}
        }

        // Listen queue of user's events to execute --------------------
        log::info!("Wainting action....");
        match action_queue_receiver.recv() {
            Ok(action_to_exec) => {
                log::info!("Action {:?}", action_to_exec);

                let command = app
                    .get_command_of_action(action_to_exec)
                    .unwrap_or(Commands::do_nothing());

                // Add Command to queue
                command_handler.add(command);

                // exec it
                let command_result = command_handler.run(&mut app);

                if let Err(e) = command_result {
                    app.get_data_store_mut()
                        .set_log_error(String::from("COMMAND ERROR"), e.to_string())
                }
            }
            Err(err) => {
                log::error!("Action ERROR");
                log::error!("{}", err);
            }
        }
    }

    input_handler.close();
    view.close();

    Ok(())
}
