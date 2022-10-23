mod app;
mod ui;
// use app::App;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let mut stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut App = ui::UI::init(&mut terminal).unwrap();

    // create app and run it
    let res = run_app(&mut App);

    // // restore terminal
    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;

    // App.close();

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(App: &mut ui::UI<B>) -> io::Result<()> {
    loop {
        App.render();

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}
