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

                let chunks = Layout::default()
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

                // Tablist
                let tabs_spans = data_store
                    .get_requests()
                    .into_iter()
                    .map(|req| Spans::from(vec![Span::from(req.name.clone())]))
                    .collect();

                let tabs = Tabs::new(tabs_spans)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title("Tabs"),
                    )
                    .style(style_if_state_is(StatesNames::TabList))
                    .select(data_store.request_ind())
                    .highlight_style(
                        Style::default()
                            .add_modifier(Modifier::BOLD)
                            .bg(Color::Black)
                            .fg(Color::LightYellow),
                    );
                f.render_widget(tabs, chunks[0]);

                // Layout geral
                let content_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .split(chunks[1]);

                // REQUEST
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

                let header_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints([Constraint::Length(7), Constraint::Min(1)].as_ref())
                    .split(request_layout[0]);

                let method = Paragraph::new("GET")
                    .style(style_if_state_is(StatesNames::Url))
                    .style(Style::default().bg(Color::Blue).fg(Color::Black))
                    .alignment(Alignment::Center);
                f.render_widget(method, header_layout[0]);

                let url_block = Block::default()
                    .borders(Borders::ALL)
                    .title("URL")
                    .title_alignment(Alignment::Left)
                    .style(style_if_state_is(StatesNames::Url))
                    .border_type(BorderType::Rounded);
                let url_text = Paragraph::new(data_store.get_request().url.clone())
                    .alignment(Alignment::Left)
                    .block(url_block.clone());
                // f.render_widget(url, header_layout[1]);
                // println!("{}", app.get_current_request().url.clone());
                f.render_widget(url_text, header_layout[1]);

                let body = Block::default()
                    .borders(Borders::ALL)
                    .title("BODY / Headers / Options")
                    .title_alignment(Alignment::Left)
                    .style(style_if_state_is(StatesNames::RequestBody))
                    .border_type(BorderType::Rounded);
                f.render_widget(body, request_layout[1]);

                // RESPONSE SECTION
                let response_block = Block::default()
                    .borders(Borders::ALL)
                    .title("Response")
                    .title_alignment(Alignment::Center)
                    .style(style_if_state_is(StatesNames::ResponseBody))
                    .style(style_if_state_is(StatesNames::ResponseHeader))
                    .border_type(BorderType::Rounded);
                f.render_widget(response_block, content_layout[1]);

                let response_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
                    .split(content_layout[1]);

                let status_code = Paragraph::new(" 200 ")
                    .style(Style::default().bg(Color::Green).fg(Color::Black))
                    .alignment(Alignment::Center);
                f.render_widget(status_code, response_layout[0]);

                let body_response = Block::default()
                    .borders(Borders::ALL)
                    .title("BODY / Headers / Options")
                    .title_alignment(Alignment::Left)
                    .style(style_if_state_is(StatesNames::ResponseBody))
                    .border_type(BorderType::Rounded);
                // f.render_widget(body_response, response_layout[1]);

                let response = data_store.get_response().clone();
                let response_data = response.lock().unwrap().clone();
                let response_text = Paragraph::new(response_data.body)
                    .alignment(Alignment::Left)
                    .block(body_response.clone());
                f.render_widget(response_text, response_layout[1]);

                // LOG SECTION
                let log_block = Block::default()
                    .borders(Borders::TOP)
                    // .style(Style::default().fg(Color::LightYellow))
                    .style(style_if_state_is(StatesNames::Log))
                    .title("Logs");
                let log_text = Paragraph::new(data_store.logs.clone())
                    .alignment(Alignment::Left)
                    .block(log_block.clone());

                // Command queue
                let log_command_queue = Paragraph::new(data_store.get_keys_queue().clone())
                    .alignment(Alignment::Right)
                    .block(log_block.clone());

                f.render_widget(log_text, chunks[2]);
                f.render_widget(log_command_queue, chunks[2]);

                // INPUT MODE
                if let InputMode::Insert = data_store.get_mode() {
                    let popup_block = Block::default()
                        .title("[ESC] - QUIT     [ENTER] - FINISH")
                        .borders(Borders::ALL);
                    let popup_text = Paragraph::new(data_store.input_buffer.buffer.clone())
                        .alignment(Alignment::Left)
                        .block(popup_block.clone());
                    let area = centered_rect(60, 10, size);
                    f.render_widget(Clear, area); //this clears out the background
                    f.render_widget(popup_text, area);
                }
            })
            .unwrap();
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
