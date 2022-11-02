#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use commands::CommandsList;
use crossterm::event::{self, Event, KeyCode};
use events::EVENTS;
use states::{default::DefaultState, State};
use std::{error::Error, io};

mod ui;
use ui::UI;

mod app;
use app::App;

mod events;
mod request;
use request::Request;

mod keymaps;
use keymaps::KeyMap;

mod commands;

mod states;

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
    let mut app_ui = UI::init();

    let default_states = DefaultState::init();
    let mut current_state = states::active_tablist::TabActiveState::init();

    loop {
        app_ui.render(&app);

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }

            let event_key = app.get_event_of_key(key.code).unwrap_or(&EVENTS::Null);
            let command = states::get_command_of_event_with_states(
                vec![&current_state.maps, &default_states.maps],
                &event_key,
            )
            .unwrap_or(CommandsList::do_nothing());

            let command_result = command(&mut app);
            if let Err(e) = command_result {
                app.set_log("Erro na execução de um comando".to_string());
            }
        }
    }

    app_ui.close();

    Ok(())
}
