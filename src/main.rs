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

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let mut app = App::default();
    app.create_request(Request::default());

    let mut app_ui: UI = UI::init(&app);

    loop {
        app_ui.render();

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                break;
            }
        }
    }

    app_ui.close();

    Ok(())
}
