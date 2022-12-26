#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use app::states::empty::EmptyState;
use app::states::manager::StateManager;
use base::actions::manager::ActionsManager;
use base::actions::Actions;
use base::commands::handler::CommandHandler;
use base::web::client::WebClient;
use base::web::repository::reqwest::ReqwestClientRepository;
use commands::Commands;
use crossterm::event::{self, Event, KeyCode};
use states::{default::DefaultState, State};
use std::{error::Error, io};

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
    let commands = default_keymap_factory();
    let keymap = KeyboardListerner {
        default: &commands,
        current: &commands,
    };
    let state_manager = StateManager::init(DefaultState::init(), EmptyState::init());
    let action_manager = ActionsManager {};
    let command_handler = CommandHandler {};

    // let web_client = WebClient::init(ReqwestClientRepository::default());

    // Init app -> start with a empty request
    let mut app = App::default();
    app.set_keymap(keymap);
    app.set_state_manager(state_manager);
    app.set_action_manager(action_manager);
    app.set_command_handler(command_handler);
    // app.set_web_client(web_client);

    app.create_request(Request::default());

    // Init UI
    let mut view = UI::init();

    loop {
        view.render(&app);

        if let Event::Key(key) = event::read()? {
            if let InputMode::Insert = app.get_mode() {
                app.edit_input_mode(key.code);
                continue;
            }

            if let KeyCode::Char('q') = key.code {
                break;
            }

            let action_to_exec = app
                .get_event_of_key(key.code)
                .unwrap_or(&Actions::Null)
                .clone();

            let command = app
                .get_command_of_action(action_to_exec)
                .unwrap_or(Commands::do_nothing())
                .clone();

            let command_result = CommandHandler::execute(&mut app, command);

            if let Err(e) = command_result {
                app.set_log("Erro na execução de um comando".to_string());
            }
        }
    }

    view.close();

    Ok(())
}
