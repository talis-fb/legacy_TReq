#[allow(dead_code)]
#[allow(unused_variables)]
use std::collections::HashMap;

use crate::app::App;

// use tui::layout::Layout as Rect;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Widget},
    Frame, Terminal,
};

// pub struct Layouts {
//     header: Rect,
//     content: Rect,
//     footer: Rect,
// }
// pub struct Component<'a> {
//     layout: &'a Rect,
//     widget: Box<dyn Widget>,
// }

pub trait UiTrait<'a> {
    fn init(app: &'a App) -> Self;
}

pub struct UI<'a> {
    // layouts: HashMap<String, Rect>,
    // components: Vec<Component>,
    terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &'a App,
}

impl<'a> UiTrait<'a> for UI<'a> {
    fn init(app: &'a App) -> Self {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap_or(());
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        UI { terminal, app }
    }
}

impl UI<'_> {
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

    pub fn render(&mut self) {
        self.terminal
            .draw(|f| {
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
                let tabs_spans = self
                    .app
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
                    .select(self.app.current_request)
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
                    .style(Style::default().bg(Color::Blue).fg(Color::Black))
                    .alignment(Alignment::Center);
                f.render_widget(method, header_layout[0]);

                let url = Block::default()
                    .borders(Borders::ALL)
                    .title("URL")
                    .title_alignment(Alignment::Left)
                    .border_type(BorderType::Rounded);
                f.render_widget(url, header_layout[1]);

                let body = Block::default()
                    .borders(Borders::ALL)
                    .title("BODY / Headers / Options")
                    .title_alignment(Alignment::Left)
                    .border_type(BorderType::Rounded);
                f.render_widget(body, request_layout[1]);

                // RESPONSE SECTION
                let response_block = Block::default()
                    .borders(Borders::ALL)
                    .title("Response")
                    .title_alignment(Alignment::Center)
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
                    .border_type(BorderType::Rounded);
                f.render_widget(body_response, response_layout[1]);

                // LOG SECTION
                let log_block = Block::default().borders(Borders::TOP).title("Logs");
                f.render_widget(log_block, chunks[2]);
            })
            .unwrap();
    }
}
