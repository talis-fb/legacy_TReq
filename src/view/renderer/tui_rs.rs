use super::{Backend, Tui};
use crate::config::configurations::view::ViewConfig;
use crate::view::style::{Color, Size, Texts};
use tui::layout::{Constraint, Direction, Layout};
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
    pub queue_render: Vec<Box<dyn FnMut(&mut Frame<CrosstermBackend<std::io::Stdout>>)>>,
}

impl BackendTuiRs {
    pub fn draw_all(&mut self) {
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

    pub fn create_absolute_centered_area(size_x: Size, size_y: Size, r: Rect) -> Rect {
        let size_x = match size_x {
            Size::Percentage(i) => [
                Constraint::Percentage((100 - i) / 2),
                Constraint::Percentage(i),
                Constraint::Percentage((100 - i) / 2),
            ],
            Size::Fixed(i) => [
                Constraint::Length((r.width / 2) - (i / 2)),
                Constraint::Length(i),
                Constraint::Length((r.width / 2) - (i / 2)),
                // Constraint::Min(1),
            ],
        };

        let size_y = match size_y {
            Size::Percentage(i) => [
                Constraint::Percentage((100 - i) / 2),
                Constraint::Percentage(i),
                Constraint::Percentage((100 - i) / 2),
            ],
            Size::Fixed(i) => [
                Constraint::Length((r.height / 2) - (i / 2)),
                Constraint::Length(i),
                Constraint::Length((r.height / 2) - (i / 2)),
            ],
        };

        let layout_y = Layout::default()
            .direction(Direction::Vertical)
            .constraints(size_y)
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints(size_x)
            .split(layout_y[1])[1]
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

        let body_block = Block::default()
            .borders(Borders::ALL)
            .title("Tabs")
            .title_alignment(Alignment::Left)
            .style(Style::default().fg(ColorTuiRs::LightYellow))
            .border_type(BorderType::Rounded);

        let area_tab = Layout::default()
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(area);

        let tabs = Tabs::new(tabs_spans)
            .style(Style::default().fg(Color::White.to_tuirs()))
            .select(current)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Black.to_tuirs())
                    .fg(Color::Yellow.to_tuirs()),
            );

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(body_block.clone(), area);
            f.render_widget(tabs.clone(), area_tab[0])
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

    fn render_text<'a>(&mut self, text: Texts, area: Rect) {
        let content = BackendTuiRs::style_span(text);
        let text = Paragraph::new(content)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(text.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_rows_texts<'a>(&mut self, text: Vec<Texts>, area: Rect) {
        let content: Vec<Spans> = text.into_iter().map(BackendTuiRs::style_span).collect();

        let text = Paragraph::new(content)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

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

    fn render_text_raw_with_cursor_at<'a>(&mut self, text: &str, cursor: usize, area: Rect) {
        let texts: Vec<Span> = if cursor == text.len() {
            vec![
                Span::from(text.to_string()),
                Span::styled(
                    " ",
                    Style::default().fg(ColorTuiRs::Black).bg(ColorTuiRs::White),
                ),
            ]
        } else {
            let (back_text, front_text) = text.split_at(cursor);
            let (character_with_cursor, front_text) = front_text.split_at(1);

            vec![
                Span::from(back_text.to_string()),
                Span::styled(
                    character_with_cursor.to_string(),
                    Style::default().fg(ColorTuiRs::Black).bg(ColorTuiRs::White),
                ),
                Span::from(front_text.to_string()),
            ]
        };

        let spans = Spans::from(texts);

        let text = Paragraph::new(spans).alignment(Alignment::Left);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(text.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_text_raw_align_center<'a>(&mut self, text: &str, area: Rect) {
        let text = Paragraph::new(text.to_string()).alignment(Alignment::Center);

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
            .title(BackendTuiRs::style_span(text))
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(block.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_clear_area(&mut self, area: Rect) {
        let closure1 =
            move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| f.render_widget(Clear, area);

        self.queue_render.push(Box::new(closure1));
    }
}

impl Backend for BackendTuiRs {}
