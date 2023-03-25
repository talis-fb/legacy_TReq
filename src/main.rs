#![allow(dead_code)]
#![allow(unused_variables)]
use base::actions::manager::ActionsManager;
use base::actions::Actions;
use base::commands::handler::CommandHandler;
use base::stores::MainStore;
use base::web::client::WebClient;
use base::web::repository::reqwest::ReqwestClientRepository;
use commands::Commands;
use config::manager::ConfigManager;
use std::error::Error;
use std::sync::Arc;

use base::states::manager::StateManager;
use base::states::states::{DefaultEditMode, DefaultHelpMode, DefaultState, State};

use std::sync::mpsc::{self, Receiver, Sender};

mod app;
use app::{App, InputMode};

mod utils;

mod input;
use input::keymaps;
use input::listener::KeyboardListerner;

mod base;
use base::{actions, commands};

mod view;
use view::ui::UI;

mod config;

mod logger;

use input::input_handler::InputHandler;
use utils::custom_types::async_bool::AsyncBool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    logger::init_logger();

    let state_manager = StateManager::init(DefaultState::init(), DefaultState::init());
    let action_manager = ActionsManager::init();
    let mut command_handler = CommandHandler::init();

    // Configurations and Setup of necessary folders
    ConfigManager::setup_env().expect("Error creating folders .local/share/treq. If error persist create it with mkdir $HOME/.local/share/treq");
    let config_manager = ConfigManager::init();
    let already_opened_app_once = config_manager
        .saved_requests
        .lock()
        .unwrap()
        .exist_already_some_file();

    // Init of Data Stores
    let mut data_store = MainStore::init(config_manager);
    data_store.set_log_warning(String::from("NEEDING HELP,"), String::from("press [?]"));

    // Init Web Client
    let web_client: WebClient<ReqwestClientRepository> =
        WebClient::init(ReqwestClientRepository::default());

    // User Input
    let (action_queue_sender, action_queue_receiver): (Sender<Actions>, Receiver<Actions>) =
        mpsc::channel();
    let has_clicked_before = Arc::new(AsyncBool::init(true));

    // Keymaps...
    // Normal Mode
    let commands = keymaps::normal_mode::keymap_factory();
    let keymap = KeyboardListerner::init(commands);

    // Input Mode
    let commands_input_mode = keymaps::input_mode::keymap_factory();
    let keymap_input_mode = KeyboardListerner::init(commands_input_mode);

    // Help Mode
    let commands_input_mode = keymaps::docs_mode::keymap_factory();
    let keymap_doc_mode = KeyboardListerner::init(commands_input_mode);

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

    if !already_opened_app_once {
        command_handler.add(Commands::open_welcome_screen());
        command_handler.run(&mut app).unwrap();
    }

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
