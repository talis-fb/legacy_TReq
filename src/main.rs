#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crossterm::event::{self, Event, KeyCode};
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
    loop {
        app_ui.render(&app);

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }

            let d = app
                .keymap
                .get_command(key.code)
                .unwrap_or(&events::EVENTS::Null);
        }
    }

    app_ui.close();

    Ok(())
}
