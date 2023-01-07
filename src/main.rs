#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use app::states::empty::EmptyState;
use app::states::manager::StateManager;
use base::actions::manager::ActionsManager;
use base::actions::Actions;
use base::commands::handler::CommandHandler;
use base::store::DataStore;
use base::web::client::WebClient;
use base::web::repository::reqwest::ReqwestClientRepository;
use commands::Commands;
use crossterm::event::{self, Event, KeyCode};
use input::buffer::InputBuffer;
use states::{default::DefaultState, State};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{error::Error, io};

use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Duration;

mod app;
mod utils;
use app::app::{App, InputMode};
use app::states;
use utils::AsyncBool;

mod input;
use input::keymaps::default_keymap_factory;
use input::listener::KeyboardListerner;

mod base;
// use base::commands;
use base::web::request::Request;
use base::{actions, commands};

mod view;
use view::ui::UI;

use input::input_handler::InputHandler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state_manager = StateManager::init(DefaultState::init(), DefaultState::init());
    let action_manager = ActionsManager {};
    let command_handler = CommandHandler {};

    let mut req = Request::default();
    req.set_url(String::from("https://test-tuirs.free.beeceptor.com"));
    req.set_name("Testezim".to_string());
    let data_store = DataStore::init(vec![req]);

    let web_client: WebClient<ReqwestClientRepository> =
        WebClient::init(ReqwestClientRepository::default());

    // Init app -> start with a empty request
    let mut app = App::default();
    app.set_state_manager(state_manager);
    app.set_action_manager(action_manager);
    app.set_command_handler(command_handler);
    app.set_web_client(web_client);
    app.set_data_store(data_store);

    // Init UI
    let mut view = UI::init();
    let delay_between_renders = Duration::from_millis(50);
    let mut interval_render = tokio::time::interval(delay_between_renders);

    // User Input
    let (action_queue_sender, action_queue_receiver): (Sender<Actions>, Receiver<Actions>) =
        mpsc::channel();
    let (typing_queue_sender, typing_queue_receiver): (Sender<Actions>, Receiver<Actions>) =
        mpsc::channel();
    let has_clicked_before = Arc::new(AsyncBool::init(true));
    let commands = default_keymap_factory();
    let keymap = KeyboardListerner::init(commands);
    let input_handler = InputHandler::init(keymap);

    while !app.is_finished {
        view.render(app.get_data_store());

        match app.get_mode() {
            InputMode::Vim => {
                view.close();

                let (new_buffer, is_finished) = input_handler.sync_open_vim(app.get_input_buffer());
                app.set_input_buffer(new_buffer);

                if is_finished {
                    view = UI::init();
                    app.clear_log();
                    app.exec_input_buffer_command();
                    app.set_mode(InputMode::Normal);
                }
            }

            InputMode::Insert => {
                let (new_buffer, is_finished) =
                    input_handler.sync_handler_typing(app.get_input_buffer());

                app.set_input_buffer(new_buffer);

                if is_finished {
                    app.clear_log();
                    app.exec_input_buffer_command();
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
                            app.get_data_store_mut().set_log_error(
                                String::from("COMMAND ERROR"),
                                e.to_string(),
                            )
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
