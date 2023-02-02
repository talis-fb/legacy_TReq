pub mod help;
pub mod ui;
pub mod renderer;
pub mod components;

use crate::base::states::names::StatesNames;
use crate::base::stores::MainStore;
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame,
};

mod drawers {
    use tui::{
        layout::{Constraint, Direction, Layout},
        widgets::{Clear, Wrap},
    };

    use crate::base::{logs::LogType, web::request::METHODS};

    use super::*;
    pub fn draw_tablist_requests<T>(frame: &mut Frame<T>, area: Rect, store: &MainStore)
    where
        T: Backend,
    {
        let req_names: Vec<(String, bool)> = store
            .get_requests()
            .into_iter()
            .map(|req| (req.name.clone(), req.has_changed))
            .collect();
        let tabs_spans = req_names
            .into_iter()
            .map(|(s, changed)| {
                Spans::from(vec![
                    Span::from(s),
                    if changed {
                        Span::from("*")
                    } else {
                        Span::from("")
                    },
                ])
            })
            .collect();

        let tabs = Tabs::new(tabs_spans)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Tabs"),
            )
            .style(if store.current_state == StatesNames::TabList {
                Style::default().fg(Color::LightYellow)
            } else {
                Style::default()
            })
            .select(store.request_ind())
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black)
                    .fg(Color::LightYellow),
            );

        frame.render_widget(tabs, area)
    }

    pub fn draw_logs_section<T>(frame: &mut Frame<T>, area: Rect, store: &MainStore)
    where
        T: Backend,
    {
        let log_block = Block::default()
            .borders(Borders::TOP)
            .style(if store.current_state == StatesNames::Log {
                Style::default().fg(Color::LightYellow)
            } else {
                Style::default()
            })
            .title("Logs");

        let type_log = store.log.log_type;
        let title_log = &store.log.title;
        let details_log = store.log.detail.as_deref().unwrap_or("");

        let style_by_type = |t: LogType| match t {
            LogType::Error => Style::default().fg(Color::Red),
            LogType::Help => Style::default().fg(Color::Blue),
            LogType::Empty => Style::default().fg(Color::Black),
            LogType::Warning => Style::default().fg(Color::Yellow),
            LogType::InputMode => Style::default().fg(Color::Cyan),
        };

        let logs = vec![Spans::from(vec![
            Span::styled(title_log, style_by_type(type_log)),
            Span::from(" "),
            Span::from(details_log),
        ])];

        let log_text = Paragraph::new(logs)
            .alignment(Alignment::Left)
            .block(log_block.clone());

        let log_command_queue = Paragraph::new(store.get_keys_queue())
            .alignment(Alignment::Right)
            .block(log_block.clone());

        frame.render_widget(log_block, area);
        frame.render_widget(log_text, area);
        frame.render_widget(log_command_queue, area);
    }

    pub fn draw_body_request_section<T>(frame: &mut Frame<T>, area: Rect, store: &MainStore)
    where
        T: Backend,
    {
        let title_in_body = vec![
            Span::styled("BODY", Style::default().fg(Color::LightYellow)),
            Span::from(" / Headers"),
        ];
        let title_in_header = vec![
            Span::from("Body / "),
            Span::styled("HEADERS", Style::default().fg(Color::LightYellow)),
        ];

        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(match store.current_state {
                StatesNames::RequestHeaders => title_in_header,
                _ => title_in_body,
            })
            .title_alignment(Alignment::Left)
            //
            .style(match store.current_state {
                StatesNames::RequestHeaders | StatesNames::RequestBody => {
                    Style::default().fg(Color::LightYellow)
                }
                _ => Style::default(),
            })
            .border_type(BorderType::Rounded);

        let content = match store.current_state {
            StatesNames::RequestBody => store.get_request().body.clone(),
            StatesNames::RequestHeaders => {
                serde_json::to_string_pretty(&store.get_request().headers).unwrap_or_default()
            }
            _ => store.get_request().body.clone(),
        };

        let body_text = Paragraph::new(content)
            .alignment(Alignment::Left)
            .block(body_block.clone());

        frame.render_widget(body_block, area);
        frame.render_widget(body_text, area);
    }

    pub fn draw_method_and_url<T>(frame: &mut Frame<T>, area: Rect, store: &MainStore)
    where
        T: Backend,
    {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Length(7), Constraint::Min(1)].as_ref())
            .split(area);

        let method = Paragraph::new(store.get_request().method.to_string())
            .style(match store.get_request().method {
                METHODS::GET => Style::default().bg(Color::Blue).fg(Color::Black),
                METHODS::POST => Style::default().bg(Color::Green).fg(Color::Black),
                METHODS::PUT => Style::default().bg(Color::White).fg(Color::Black),
                METHODS::PATCH => Style::default().bg(Color::Magenta).fg(Color::Black),
                METHODS::DELETE => Style::default().bg(Color::Red).fg(Color::Black),
                METHODS::HEAD => Style::default().bg(Color::Yellow).fg(Color::Black),
            })
            .alignment(Alignment::Center);
        frame.render_widget(method, layout[0]);

        let url_block = Block::default()
            .borders(Borders::ALL)
            .title("URL")
            .title_alignment(Alignment::Left)
            .style(if store.current_state == StatesNames::Url {
                Style::default().fg(Color::LightYellow)
            } else {
                Style::default()
            })
            .border_type(BorderType::Rounded);

        let url_text = Paragraph::new(store.get_request().url.clone())
            .alignment(Alignment::Left)
            .block(url_block.clone());

        frame.render_widget(url_text, layout[1]);
    }

    pub fn draw_body_response_section<T>(frame: &mut Frame<T>, area: Rect, store: &MainStore)
    where
        T: Backend,
    {
        let title_in_body = vec![
            Span::styled("BODY", Style::default().fg(Color::LightYellow)),
            Span::from(" / Headers"),
        ];
        let title_in_header = vec![
            Span::from("Body / "),
            Span::styled("HEADERS", Style::default().fg(Color::LightYellow)),
        ];

        // RESPONSE SECTION
        let response_block = Block::default()
            .borders(Borders::ALL)
            .title("Response")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let response_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
            .split(area);

        let response = store.get_response();
        let response_data = response.lock().unwrap().clone();

        let status = response_data.status;
        let content = match store.current_state {
            StatesNames::ResponseHeader => {
                serde_json::to_string_pretty(&response_data.headers).unwrap_or_default()
            }
            _ => response_data.body,
        };

        let status_code = Paragraph::new(match status {
            0 => String::from("Hit ENTER to submit"),
            77 => String::from("Error"), // A STATUS CODE INTERNAL TO INTERNAL ERROR
            _ => status.to_string(),
        })
        .style(match status {
            0 => Style::default().bg(Color::Gray).fg(Color::Black),
            77 => Style::default().bg(Color::Red).fg(Color::Black), // A STATUS CODE INTERNAL TO INTERNAL ERROR
            100..=199 => Style::default().bg(Color::Gray).fg(Color::Black),
            200..=299 => Style::default().bg(Color::Green).fg(Color::Black),
            300..=399 => Style::default().bg(Color::Yellow).fg(Color::Black),
            400..=499 => Style::default().bg(Color::Magenta).fg(Color::Black),
            500..=599 => Style::default().bg(Color::LightRed).fg(Color::Black),
            _ => Style::default().bg(Color::Cyan).fg(Color::Black),
        })
        .alignment(Alignment::Center);

        let body_response = Block::default()
            .borders(Borders::ALL)
            .title(match store.current_state {
                StatesNames::ResponseHeader => title_in_header,
                _ => title_in_body,
            })
            .title_alignment(Alignment::Left)
            .style(match store.current_state {
                StatesNames::ResponseHeader | StatesNames::ResponseBody => {
                    Style::default().fg(Color::LightYellow)
                }
                _ => Style::default(),
            })
            .border_type(BorderType::Rounded);

        let response_text = Paragraph::new(content)
            .alignment(Alignment::Left)
            .block(body_response.clone());

        frame.render_widget(response_block, area);
        frame.render_widget(status_code, response_layout[0]);
        frame.render_widget(response_text, response_layout[1]);
    }

    pub fn draw_input_popup<T>(frame: &mut Frame<T>, area: Rect, store: &MainStore)
    where
        T: Backend,
    {
        let popup_block = Block::default()
            .title("[ESC] - QUIT     [ENTER] - FINISH")
            .borders(Borders::ALL);
        let popup_text = Paragraph::new(store.input_buffer.value.clone())
            .alignment(Alignment::Left)
            .block(popup_block.clone());
        let popup_area = centered_rect(60, 10, area);

        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_text, popup_area);
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

    pub fn draw_help_popup<T>(frame: &mut Frame<T>, area: Rect, store: &MainStore)
    where
        T: Backend,
    {
        // TODO:
        // This verification of Vec<Span> should not be done here!
        //
        let doc_handler = store.doc_reader.as_ref().unwrap();
        let mut content = doc_handler.get_doc_spans();
        let position = doc_handler.position;

        if position >= content.len() {
            content.clear();
        } else {
            let (_, content) = content.split_at(position);
        }

        let popup_block = Block::default()
            .title("Navigate -> [UP] and [DOWN] / Press any other key to close")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightYellow))
            .title_alignment(Alignment::Center);

        let popup_text = Paragraph::new(content.to_vec())
            .alignment(Alignment::Left)
            .block(popup_block.clone())
            .wrap(Wrap { trim: true });
        let popup_area = centered_rect(60, 75, area);

        frame.render_widget(Clear, popup_area);
        frame.render_widget(popup_text, popup_area);
    }
}
