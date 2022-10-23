use crossterm::event::{self, Event, KeyCode};
use std::{error::Error, io};

mod ui;
use ui::UI;

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let mut app_ui = UI::default();

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
