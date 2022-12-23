#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use app::states::empty::EmptyState;
use app::states::manager::StateManager;
use base::actions::Actions;
use base::actions::manager::ActionsManager;
use base::commands::handler::CommandHandler;
use commands::Commands;
use crossterm::event::{self, Event, KeyCode};
use states::{default::DefaultState, State};
use std::{error::Error, io};

mod app;
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

fn main() -> Result<(), Box<dyn Error>> {
    // setup commands keys
    let commands = default_keymap_factory();
    let keymap = KeyboardListerner {
        default: &commands,
        current: &commands,
    };

    // Start App Dependecy
    let state_manager = StateManager::init(DefaultState::init(), EmptyState::init());
    let action_manager = ActionsManager {};
    let command_handler = CommandHandler {};

    // Init app -> start with a empty request
    let mut app = App::init(keymap, state_manager, action_manager, command_handler);
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
                .action_manager
                .get_command_of_action(action_to_exec, &app.state_manager)
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
