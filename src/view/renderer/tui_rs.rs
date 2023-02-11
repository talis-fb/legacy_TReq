use super::{Backend, Tui};
use crate::base::doc::handler::DocReaderHandler;
use crate::config::configurations::view::ViewConfig;
use crate::view::style::Texts;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Clear, Wrap};
use tui::{backend::CrosstermBackend, layout::Rect};
use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
};
use tui::{Frame, Terminal};

// use std::default::default;
use std::ops::FnMut;

pub struct BackendTuiRs {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub configs: ViewConfig,

    // TODO: make it private
    pub queue_render: Vec<Box<dyn FnMut(&mut Frame<CrosstermBackend<std::io::Stdout>>) -> ()>>, // pub queue_render: Vec<(Box<dyn Widget>, Rect)>
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
                    Span::styled(f.body.to_string(), Style::default().fg(Color::Red))
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
                    .bg(Color::Black)
                    .fg(Color::LightYellow),
            );

        // self.terminal.draw(|f|{
        //     f.render_widget(tabs, area)
        // });

        // self.queue_render.push(RenderProcess::init(Box::new(tabs), area));
        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(tabs.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_block_with_title_left(&mut self, title: Texts, area: Rect) {
        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        // self.terminal.draw(|f|{
        //     f.render_widget(body_block, area)
        // });
        // self.queue_render.push(RenderProcess::init(Box::new(body_block), area));
        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(body_block.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_block_with_title_center(&mut self, title: Texts, area: Rect) {
        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        // self.terminal.draw(|f|{
        //     f.render_widget(body_block, area)
        // });
        // self.queue_render
        //     .push(RenderProcess::init(Box::new(body_block), area));

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
                current if current > 0 => {
                    Span::styled(content.to_string(), Style::default().fg(Color::LightYellow))
                }
                0 => Span::from(content.to_string()),
                _ => Span::from(content.to_string()),
            })
            .collect();

        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(tabs_spans)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        // self.terminal.draw(|f|{
        // f.render_widget(body_block, area)
        // });
        // self.queue_render
        //     .push(RenderProcess::init(Box::new(body_block), area));

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(body_block.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_help_window<'a>(&mut self, doc_handler: &'a DocReaderHandler, area: Rect) {
        // TODO
        // TODO: Not passing Spans to queue_render (because Cow inside it)
        // TODO

        // let mut content = doc_handler.doc.to_vec_spans().clone();
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
            .border_style(Style::default().fg(Color::LightYellow))
            .title_alignment(Alignment::Center);

        let popup_text = Paragraph::new(content)
            .alignment(Alignment::Left)
            // .block(popup_block)
            .wrap(Wrap { trim: true });

        let popup_area = BackendTuiRs::centered_rect(60, 75, area);

        // self.terminal.draw(|f| f.render_widget(Clear, popup_area));
        // self.terminal.draw(|f| f.render_widget(popup_text, popup_area));

        // let aaa = Box::new(popup_text);
        // self.queue_render.push(RenderProcess::init(Box::new(Clear), popup_area));
        // self.queue_render.push(RenderProcess::init(aaa, popup_area));

        let closure1 = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(Clear, popup_area)
        };

        // let closure2 = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
        //     f.render_widget(popup_text, popup_area)
        // };

        self.queue_render.push(Box::new(closure1));
        // self.queue_render.push(Box::new(closure2));
    }

    fn render_text<'a>(&mut self, text: Texts, area: Rect) {
        let text = Paragraph::new(text.to_string()).alignment(Alignment::Left);

        // self.terminal.draw(|f| f.render_widget(text, area));
        // self.queue_render
        //     .push(RenderProcess::init(Box::new(text), area));

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(text.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    fn render_text_in_block<'a>(&mut self, block_title: Texts, text: Texts, area: Rect) {
        let spans = BackendTuiRs::style_span(block_title);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(spans)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        let text = Paragraph::new(text.to_string())
            .alignment(Alignment::Left)
            .block(block);

        let closure = move |f: &mut Frame<CrosstermBackend<std::io::Stdout>>| {
            f.render_widget(text.clone(), area)
        };

        self.queue_render.push(Box::new(closure));
    }

    // TODO: Add Custom Style
    fn render_text_with_bg<'a>(&mut self, text: Texts, area: Rect) {
        let block = Paragraph::new(text.to_string())
            .style(Style::default().bg(Color::Blue).fg(Color::Black))
            .alignment(Alignment::Center);

        // self.terminal.draw(|f| f.render_widget(block, area));

        // self.queue_render
        //     .push(RenderProcess::init(Box::new(block), area));

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
