use std::{
    collections::HashMap,
    iter::Fuse,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
    time::Duration,
};

use crate::{
    app::app::{App, InputMode},
    base::store::DataStore,
    states::{active_tablist::TabActiveState, StatesNames},
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tokio::task::JoinHandle;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Tabs, Widget},
    Frame, Terminal,
};

use std::sync::mpsc::{self, Receiver, Sender};

use std::sync::Arc;

use crate::view::Drawers;

pub struct UI {
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl UI {
    pub fn init() -> Self {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap_or(());
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        UI { terminal }
    }

    pub fn close(&mut self) -> () {
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        self.terminal.show_cursor().unwrap();
    }

    pub fn render(&mut self, data_store: &DataStore) {
        self.terminal
            .draw(|f| {
                let current_state = data_store.current_state.clone();
                let style_if_state_is = |state: StatesNames| {
                    if state == current_state {
                        Style::default().fg(Color::LightYellow)
                    } else {
                        Style::default()
                    }
                };

                // ---------------

                let size = f.size();

                let full_screen_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            // Request List Tab
                            Constraint::Length(3),
                            Constraint::Min(1),
                            Constraint::Length(2),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());

                // Layout request + response
                let content_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(full_screen_layout[1]);

                // REQUEST BLOCK
                let request_block = Block::default()
                    .borders(Borders::ALL)
                    .title("Request")
                    .title_alignment(Alignment::Center)
                    .style(style_if_state_is(StatesNames::RequestBody))
                    .style(style_if_state_is(StatesNames::RequestHeaders))
                    .border_type(BorderType::Rounded);
                f.render_widget(request_block, content_layout[0]);

                let request_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
                    .split(content_layout[0]);

                // Tablit
                Drawers::draw_tablist_requests(f, full_screen_layout[0], data_store);

                // Request
                Drawers::draw_method_and_url(f, request_layout[0], data_store);
                Drawers::draw_body_request_section(f, request_layout[1], data_store);

                // Response
                Drawers::draw_body_response_section(f, content_layout[1], data_store);

                // Logs
                Drawers::draw_logs_section(f, full_screen_layout[2], data_store);

                // INPUT MODE
                if let InputMode::Insert = data_store.get_mode() {
                    Drawers::draw_input_popup(f, f.size(), data_store);
                }
            })
            .unwrap();
    }
}
