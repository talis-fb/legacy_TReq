#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crossterm::event::{self, Event, KeyCode};
use states::{DefaultState, State};
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
    app.create_request(Request::default());
    app.create_request(Request::default());
    app.create_request(Request::default());

    // Init UI
    let mut app_ui = UI::init();

    let states = DefaultState::init();

    loop {
        app_ui.render(&app);

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }

            let event_key = app
                .keymap
                .get_command(key.code)
                .unwrap_or(&events::EVENTS::Null);

            let dd =
                <states::DefaultState as State>::get_command_of_event(&states.maps, &event_key)
                    .unwrap_or(|app: &mut App| Ok(()));
            let res = dd(&mut app);
            match res {
                Ok(i) => {}
                Err(i) => {}
            }
        }
    }

    app_ui.close();

    Ok(())
}
