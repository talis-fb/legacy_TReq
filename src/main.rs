#![allow(dead_code)]
#![allow(unused_variables)]
use std::error::Error;
use std::sync::Arc;
use treq::base::actions::manager::ActionsManager;
use treq::base::actions::Actions;
use treq::base::commands::handler::CommandHandler;
use treq::base::commands::Commands;
use treq::base::os::file_facades::FileFacade;
use treq::base::os::file_facades::variables::VariablesFile;
use treq::base::stores::MainStore;
use treq::base::web::client::WebClient;
use treq::base::web::repository::reqwest::ReqwestClientRepository;
use treq::config::manager::ConfigManager;

use treq::base::states::manager::StateManager;
use treq::base::states::states::{DefaultEditMode, DefaultHelpMode, DefaultState, State};

use std::sync::mpsc::{self, Receiver, Sender};

use treq::app::{App, InputMode};

use treq::input::keymaps;
use treq::input::listener::KeyboardListerner;

use treq::view::ui::UI;

use treq::input::input_handler::InputHandler;
use treq::utils::custom_types::async_bool::AsyncBool;

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

    let variables_file = VariablesFile::create("global_variables".to_string(), value)

    let state_manager = StateManager::init(DefaultState::init(), DefaultState::init());
    let action_manager = ActionsManager::init();
    let mut command_handler = CommandHandler::init();

    // Configurations and Setup of necessary folders
    ConfigManager::setup_env().expect("Error creating folders .local/share/treq. If error persist create it with mkdir $HOME/.local/share/treq");
    let config_manager = ConfigManager::init();

    // Init of Data Stores
    let mut data_store = MainStore::init(config_manager);
    data_store.set_log_warning(String::from("NEEDING HELP,"), String::from("press [?]"));

    // Init Web Client
    let web_client: WebClient<ReqwestClientRepository> =
        WebClient::init(ReqwestClientRepository::default());

    // User Input
    let (action_queue_sender, action_queue_receiver): (Sender<Actions>, Receiver<Actions>) =
        mpsc::channel();

    // Keymaps...
    // Normal Mode
    let commands = keymaps::normal_mode::keymap_factory();
    let keymap = KeyboardListerner::init(commands);

    let mut input_handler = InputHandler::init(
        keymap,
        data_store.config.editor.clone(),
        data_store.config.edition_files_handler.clone(),
        action_queue_sender.clone(),
    );

    // Init UI
    let mut view = UI::init();

    // Init app -> it starts with a empty request
    let mut app = App::default();
    app.set_state_manager(state_manager);
    app.set_action_manager(action_manager);
    app.set_web_client(web_client);
    app.set_data_store(data_store);
    app.set_renderer(action_queue_sender);

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
