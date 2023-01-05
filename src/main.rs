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

mod input;
use input::keymaps::default_keymap_factory;
use input::listener::KeyboardListerner;

mod base;
// use base::commands;
use base::web::request::Request;
use base::{actions, commands};

mod view;
use view::ui::UI;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // App Dependecies
    // let keymap = KeyboardListerner {
    //     default: &commands,
    //     current: &commands,
    // };
    // let keymap = Arc::new(KeyboardListerner::init(commands));

    let state_manager = StateManager::init(DefaultState::init(), DefaultState::init());
    let action_manager = ActionsManager {};
    let command_handler = CommandHandler {};

    let mut req = Request::default();
    req.set_url(String::from("https://test-tuirs.free.beeceptor.com"));
    req.set_name("Testezim");
    let data_store = DataStore::init(vec![req]);

    let web_client: WebClient<ReqwestClientRepository> =
        WebClient::init(ReqwestClientRepository::default());

    // Init app -> start with a empty request
    let mut app = App::default();
    // app.set_keymap(*(keymap.clone()));
    app.set_state_manager(state_manager);
    app.set_action_manager(action_manager);
    app.set_command_handler(command_handler);
    app.set_web_client(web_client);
    app.set_data_store(data_store);

    // Init UI
    let view = UI::init();

    let is_finished = Arc::new(AtomicBool::new(false));

    let (tx, rx): (Sender<Actions>, Receiver<Actions>) = mpsc::channel();

    let delay_between_renders = Duration::from_millis(20);
    let mut interval_render = tokio::time::interval(delay_between_renders);

    let mut has_clicked = true;

    loop {
        let output_view = view.renderer.send(app.get_data_store().clone());
        if let Err(e) = output_view {
            println!("Erro render");
            println!("{}", e);
            break;
        }

        if let InputMode::Insert = app.get_mode() {
            if let Event::Key(key) = event::read()? {
                app.edit_input_mode(key.code);
            }
            continue;
        }

        if is_finished.load(Ordering::SeqCst) {
            break;
        }

        // <<<<<<<<<<<<<<<<<<<<<<<<<<<<
        if has_clicked {
            let aa = tx.clone();
            let is_finished_thread = is_finished.clone();
            let reading = tokio::task::spawn(async move {
                let commands = default_keymap_factory();
                let mut keymap = KeyboardListerner::init(commands);

                if let Event::Key(key) = event::read().unwrap() {
                    // IT MUST BE A ACTION
                    if let KeyCode::Char('q') = key.code {
                        is_finished_thread.store(true, Ordering::SeqCst);
                        return;
                    }

                    let action = keymap.get_command(key.code);

                    if let Some(act) = action {
                        aa.send(act);
                    }
                }
            });
            has_clicked = false;
        }
        // <<<<<<<<<<<<<<<<<<<<<<<<<<<<

        interval_render.tick().await;

        match rx.recv_timeout(delay_between_renders) {
            Ok(action_to_exec) => {
                let command = app
                    .get_command_of_action(action_to_exec)
                    .unwrap_or(Commands::do_nothing())
                    .clone();

                let command_result = CommandHandler::execute(&mut app, command);

                if let Err(e) = command_result {
                    app.set_log("Erro na execução de um comando".to_string());
                }

                has_clicked = true;
            }
            Err(_) => {}
        }
        // >>>>>>>>>>>>>>>>>>>.

        // }
    }

    view.is_finished.store(true, Ordering::SeqCst);
    let exit_output = view.thread.await;

    if let Err(e) = exit_output {
        println!("ERROR: Closing UI");
        println!("{}", e);
    }

    Ok(())
}
