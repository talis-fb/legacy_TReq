pub mod components;
pub mod ui;

use crate::{app::states::StatesNames, base::store::DataStore};
use tui::{
    backend::Backend,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
    Frame,
};

mod Drawers {
    use tui::{
        layout::{Constraint, Direction, Layout},
        widgets::Clear,
    };

    use super::*;
    pub fn draw_tablist_requests<T>(frame: &mut Frame<T>, area: Rect, store: &DataStore) -> ()
    where
        T: Backend,
    {
        let list: Vec<String> = store
            .get_requests()
            .into_iter()
            .map(|req| req.name.clone())
            .collect();
        let tabs_spans = list
            .into_iter()
            .map(|s| Spans::from(vec![Span::from(s)]))
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

    pub fn draw_logs_section<T>(frame: &mut Frame<T>, area: Rect, store: &DataStore) -> ()
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

        let log_text = Paragraph::new(store.logs.clone())
            .alignment(Alignment::Left)
            .block(log_block.clone());

        // Command queue
        let log_command_queue = Paragraph::new(store.get_keys_queue().clone())
            .alignment(Alignment::Right)
            .block(log_block.clone());

        frame.render_widget(log_block, area);
        frame.render_widget(log_command_queue, area);
    }

    pub fn draw_body_request_section<T>(frame: &mut Frame<T>, area: Rect, store: &DataStore) -> ()
    where
        T: Backend,
    {
        let body_block = Block::default()
            .borders(Borders::ALL)
            .title("BODY / Headers / Options")
            .title_alignment(Alignment::Left)
            .style(if store.current_state == StatesNames::RequestBody {
                Style::default().fg(Color::LightYellow)
            } else {
                Style::default()
            })
            .border_type(BorderType::Rounded);

        let body = store.get_request().body.clone();
        let body_text = Paragraph::new(body)
            .alignment(Alignment::Left)
            .block(body_block.clone());

        frame.render_widget(body_block, area);
        frame.render_widget(body_text, area);
    }

    pub fn draw_method_and_url<T>(frame: &mut Frame<T>, area: Rect, store: &DataStore) -> ()
    where
        T: Backend,
    {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Length(7), Constraint::Min(1)].as_ref())
            .split(area);

        let method = Paragraph::new("GET")
            .style(if store.current_state == StatesNames::Url {
                Style::default().fg(Color::LightYellow)
            } else {
                Style::default()
            })
            .style(Style::default().bg(Color::Blue).fg(Color::Black))
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

    pub fn draw_body_response_section<T>(frame: &mut Frame<T>, area: Rect, store: &DataStore) -> ()
    where
        T: Backend,
    {
        // RESPONSE SECTION
        let response_block = Block::default()
            .borders(Borders::ALL)
            .title("Response")
            .title_alignment(Alignment::Center)
            .style(if store.current_state == StatesNames::ResponseHeader {
                Style::default().fg(Color::LightYellow)
            } else {
                Style::default()
            })
            .border_type(BorderType::Rounded);

        let response_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
            .split(area);

        let status_code = Paragraph::new(" 200 ")
            .style(Style::default().bg(Color::Green).fg(Color::Black))
            .alignment(Alignment::Center);

        let body_response = Block::default()
            .borders(Borders::ALL)
            .title("BODY / Headers / Options")
            .title_alignment(Alignment::Left)
            .style(if store.current_state == StatesNames::ResponseBody {
                Style::default().fg(Color::LightYellow)
            } else {
                Style::default()
            })
            .border_type(BorderType::Rounded);

        let response = store.get_response().clone();
        let response_data = response.lock().unwrap().clone();
        let response_text = Paragraph::new(response_data.body)
            .alignment(Alignment::Left)
            .block(body_response.clone());

        frame.render_widget(response_block, area);
        frame.render_widget(status_code, response_layout[0]);
        frame.render_widget(response_text, response_layout[1]);
    }

    pub fn draw_input_popup<T>(frame: &mut Frame<T>, area: Rect, store: &DataStore) -> ()
    where
        T: Backend,
    {
        let popup_block = Block::default()
            .title("[ESC] - QUIT     [ENTER] - FINISH")
            .borders(Borders::ALL);
        let popup_text = Paragraph::new(store.input_buffer.buffer.clone())
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
}
