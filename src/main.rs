#![allow(dead_code)]
#![allow(unused_variables)]
use base::actions::manager::ActionsManager;
use base::actions::Actions;
use base::commands::handler::CommandHandler;
use base::stores::requests::RequestStore;
use base::stores::MainStore;
use base::web::client::WebClient;
use base::web::repository::reqwest::ReqwestClientRepository;
use commands::Commands;
use config::configurations::external_editor::ExternalEditor;
use config::configurations::save_files::SaveFiles;
use config::configurations::Configuration;
use config::manager::ConfigManager;
use std::error::Error;
use std::sync::Arc;

use base::states::manager::StateManager;
use base::states::states::{DefaultState, State};

use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

mod app;
use app::{App, InputMode};

mod utils;

mod input;
use input::keymaps::default_keymap_factory;
use input::listener::KeyboardListerner;

mod base;
use base::{actions, commands};

mod view;
use view::ui::UI;

mod config;

use input::input_handler::InputHandler;
use utils::custom_types::async_bool::AsyncBool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state_manager = StateManager::init(DefaultState::init(), DefaultState::init());
    let action_manager = ActionsManager {};
    let command_handler = CommandHandler {};

    // Configurations and Setup of necessary folders
    ConfigManager::setup_env().expect("Error creating folders .local/share/treq. If error persist create it with mkdir $HOME/.local/share/treq");
    let saved_requests = SaveFiles::setup_and_init().unwrap();
    let editor = ExternalEditor::setup_and_init().unwrap();
    let config_manager = ConfigManager {
        saved_requests,
        editor,
    };

    // Init of Data Stores
    let request_store = RequestStore::init(config_manager.saved_requests);
    let mut data_store = MainStore::init(request_store);
    data_store.set_log_warning(String::from("NEEDING HELP,"), String::from("press [?]"));

    // Init Web Client
    let web_client: WebClient<ReqwestClientRepository> =
        WebClient::init(ReqwestClientRepository::default());

    // User Input
    let (action_queue_sender, action_queue_receiver): (Sender<Actions>, Receiver<Actions>) =
        mpsc::channel();
    let (typing_queue_sender, typing_queue_receiver): (Sender<Actions>, Receiver<Actions>) =
        mpsc::channel();
    let has_clicked_before = Arc::new(AsyncBool::init(true));
    let commands = default_keymap_factory();
    let keymap = KeyboardListerner::init(commands);
    let input_handler = InputHandler::init(keymap, config_manager.editor);

    // Init UI
    let mut view = UI::init();
    let delay_between_renders = Duration::from_millis(50);
    let mut interval_render = tokio::time::interval(delay_between_renders);

    // Init app -> it starts with a empty request
    let mut app = App::default();
    app.set_state_manager(state_manager);
    app.set_action_manager(action_manager);
    app.set_command_handler(command_handler);
    app.set_web_client(web_client);
    app.set_data_store(data_store);

    while !app.is_finished {
        view.render(app.get_data_store());

        match app.get_mode() {
            InputMode::Help => {
                let doc_reader = app.get_data_store_mut().doc_reader.as_mut().unwrap();
                let (i, is_finished) =
                    input_handler.sync_handler_doc_reading(doc_reader.get_position() as i32);

                doc_reader.position = i;

                if is_finished {
                    app.set_mode(InputMode::Normal);
                }
            }

            InputMode::Vim => {
                view.close();

                let (new_buffer, is_finished) =
                    input_handler.sync_open_vim(app.get_input_buffer_value());
                app.set_input_buffer_value(new_buffer);

                if is_finished {
                    view = UI::init();
                    app.clear_log();
                    app.exec_input_buffer_command()?;
                    app.set_mode(InputMode::Normal);
                }
            }

            InputMode::Insert => {
                let (new_buffer_value, is_finished) =
                    input_handler.sync_handler_typing(app.get_input_buffer_mut());

                app.set_input_buffer_value(new_buffer_value);

                if is_finished {
                    app.clear_log();
                    app.exec_input_buffer_command()?;
                    app.set_mode(InputMode::Normal);
                }
            }

            InputMode::Normal => {
                // Init listener of user input if previous one had done --------
                if has_clicked_before.get() {
                    input_handler
                        .async_handler(action_queue_sender.clone(), has_clicked_before.clone());
                    has_clicked_before.set(false);
                }

                // Listen queue of user's events to execute --------------------
                interval_render.tick().await;
                match action_queue_receiver.recv_timeout(delay_between_renders) {
                    Ok(action_to_exec) => {
                        let command = app
                            .get_command_of_action(action_to_exec)
                            .unwrap_or(Commands::do_nothing())
                            .clone();

                        let command_result = CommandHandler::execute(&mut app, command);

                        if let Err(e) = command_result {
                            app.get_data_store_mut()
                                .set_log_error(String::from("COMMAND ERROR"), e.to_string())
                        }
                    }
                    Err(_) => {}
                }
            }
        }
    }

    view.close();

    Ok(())
}
