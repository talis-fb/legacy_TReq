#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use commands::Commands;
use crossterm::event::{self, Event, KeyCode};
use base::actions::Actions;
use states::{default::DefaultState, State};
use std::{error::Error, io};

// mod 
mod app;
use app::states;
use app::app::{App, InputMode};

mod input;
use input::keymaps::KeyMap;

mod base;
use base::{commands, actions};
use base::web::request::Request;

mod view;
use view::ui::UI;

fn main() -> Result<(), Box<dyn Error>> {
    // setup commands keys
    let commands = KeyMap::default_commandmap();
    let keymap = KeyMap {
        default: &commands,
        current: &commands,
    };

    // Init app
    let mut app = App::init(keymap);
    // Start with a empty request
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

            let event_key = app
                .get_event_of_key(key.code)
                .unwrap_or(&Actions::Null)
                .clone();

            let command = states::get_command_of_event_with_states(
                vec![app.current_state.get_map(), app.default_state.get_map()],
                &event_key,
            )
            .unwrap_or(Commands::do_nothing());

            let command_result = command(&mut app);
            if let Err(e) = command_result {
                app.set_log("Erro na execução de um comando".to_string());
            }
        }
    }

    view.close();

    Ok(())
}
