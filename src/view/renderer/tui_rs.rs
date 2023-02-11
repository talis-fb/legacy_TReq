use super::{Backend, Tui};
use crate::base::doc::handler::DocReaderHandler;
use crate::config::configurations::view::ViewConfig;
use crate::view::style::{Color, Texts};
use tui::layout::{Constraint, Direction, Layout};
use tui::text::Text;
use tui::widgets::{Clear, Wrap};
use tui::{backend::CrosstermBackend, layout::Rect};
use tui::{
    layout::Alignment,
    style::{Color as ColorTuiRs, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
};
use tui::{Frame, Terminal};

use std::ops::FnMut;

pub struct BackendTuiRs {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub configs: ViewConfig,

    // TODO: make it private
    pub queue_render: Vec<Box<dyn FnMut(&mut Frame<CrosstermBackend<std::io::Stdout>>) -> ()>>,
}

impl BackendTuiRs {
    pub fn draw_all(&mut self) -> () {
        let queue = &mut self.queue_render;

        self.terminal
            .draw(|f| {
                for render_func in queue.iter_mut() {
                    render_func(f)
                }
            })
            .unwrap();

        queue.clear();
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

    fn style_span<'a>(texts: Texts) -> Spans<'a> {
        let spans: Vec<Span> = texts
            .body
            .iter()
            .map(|f| {
                if let Some(style) = &f.style {
                    Span::styled(
                        f.body.to_string(),
                        Style::default().fg(style.color.to_tuirs()),
                    )
                } else {
                    Span::from(f.body.to_string())
                }
            })
            .collect();

        Spans(spans)
    }
}

impl Tui<Rect> for BackendTuiRs {
    fn render_tablist(&mut self, tabs: Vec<Texts>, current: usize, area: Rect) {
        let tabs_spans = tabs
            .into_iter()
            .map(|s| Spans::from(vec![Span::from(s.to_string())]))
            .collect();

        let tabs = Tabs::new(tabs_spans)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Tabs"),
            )
            .select(current)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black.to_tuirs())
                    .fg(Color::Yellow.to_tuirs()),
            );

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(tabs.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_tablist_marked(&mut self, tabs: Vec<Texts>, current: usize, area: Rect) {
        let tabs_spans = tabs
            .into_iter()
            .map(|s| Spans::from(vec![Span::from(s.to_string())]))
            .collect();

        let tabs = Tabs::new(tabs_spans)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .title("Tabs"),
            )
            .select(current)
            .style(Style::default().fg(ColorTuiRs::LightYellow))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black.to_tuirs())
                    .fg(Color::Yellow.to_tuirs()),
            );

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(tabs.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_block_with_title_left(&mut self, title: Texts, area: Rect) {
        let spans = BackendTuiRs::style_span(title);

        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(spans)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(body_block.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_block_with_title_center(&mut self, title: Texts, area: Rect) {
        let spans = BackendTuiRs::style_span(title);

        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(spans)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(body_block.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_block_with_tab(&mut self, tabs: Vec<Texts>, current: usize, area: Rect) {
        let tabs_spans: Vec<Span> = tabs
            .into_iter()
            .enumerate()
            .map(|(i, content)| match i {
                current if current > 0 => Span::styled(
                    content.to_string(),
                    Style::default().fg(Color::Yellow.to_tuirs()),
                ),
                0 => Span::from(content.to_string()),
                _ => Span::from(content.to_string()),
            })
            .collect();

        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(tabs_spans)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(body_block.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_block_with_title_left_marked(&mut self, title: Texts, area: Rect) {
        let spans = BackendTuiRs::style_span(title);

        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(spans)
            .title_alignment(Alignment::Left)
            .style(Style::default().fg(ColorTuiRs::LightYellow))
            .border_type(BorderType::Rounded);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(body_block.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_block_with_title_center_marked(&mut self, title: Texts, area: Rect) {
        let spans = BackendTuiRs::style_span(title);

        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(spans)
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(ColorTuiRs::LightYellow))
            .border_type(BorderType::Rounded);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(body_block.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_help_window<'a>(&mut self, doc_handler: &'a DocReaderHandler, area: Rect) {
        let mut content = doc_handler.get_doc_spans();

        let position = doc_handler.position.clone();

        if position >= content.len() {
            content.clear();
        } else {
            let (_, content) = content.split_at(position);
        }

        let popup_block = Block::default()
            .title("Navigate -> [UP] and [DOWN] / Press any other key to close")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow.to_tuirs()))
            .title_alignment(Alignment::Center);

        let popup_text = Paragraph::new(content)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        let popup_area = BackendTuiRs::centered_rect(60, 75, area);

        let closure1 = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(Clear, popup_area)
        };

        self.queue_render.push(Box::new(closure1));
    }

    fn render_text<'a>(&mut self, text: Texts, area: Rect) {
        let content = BackendTuiRs::style_span(text);
        let text = Paragraph::new(content).alignment(Alignment::Left);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(text.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }
    fn render_text_raw<'a>(&mut self, text: &str, area: Rect) {
        let text = Paragraph::new(text.to_string()).alignment(Alignment::Left);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(text.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    // TODO: Add Custom Style
    fn render_text_with_bg<'a>(&mut self, text: Texts, color: Color, area: Rect) {
        let block = Paragraph::new(text.to_string())
            .style(
                Style::default()
                    .bg(color.to_tuirs())
                    .fg(Color::Black.to_tuirs()),
            )
            .alignment(Alignment::Center);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(block.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_bg_color<'a>(&mut self, color: Color, area: Rect) {
        let block = Paragraph::new("")
            .style(Style::default().bg(color.to_tuirs()))
            .alignment(Alignment::Center);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(block.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_divider_with_text(&mut self, text: Texts, area: Rect) {
        let block = Block::default()
            .borders(Borders::TOP)
            .title(text.to_string())
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(block.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }
}

impl Backend for BackendTuiRs {}
