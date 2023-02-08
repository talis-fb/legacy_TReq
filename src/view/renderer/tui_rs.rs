use super::{Backend, Tui};
use crate::base::doc::handler::DocReaderHandler;
use crate::config::configurations::view::ViewConfig;
use tui::Terminal;
use tui::layout::{Layout, Direction, Constraint};
use tui::widgets::{Wrap, Clear};
use tui::{backend::CrosstermBackend, layout::Rect};
use tui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs},
};

pub struct BackendTuiRs {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    pub configs: ViewConfig,
}

impl BackendTuiRs {
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

impl Tui<Rect> for BackendTuiRs {
    fn render_tablist(&mut self, tabs: Vec<&str>, current: usize, area: Rect) {
        let tabs_spans = tabs
            .into_iter()
            .map(|s| {
                Spans::from(vec![ Span::from(s) ])
            })
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

        self.terminal.draw(|f|{
            f.render_widget(tabs, area)
        });
    }


    fn render_block_with_title_left(&mut self, title: &str, area: Rect) {
        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_alignment(Alignment::Left)
            .border_type(BorderType::Rounded);

        self.terminal.draw(|f|{
            f.render_widget(body_block, area)
        });
    }

    fn render_block_with_title_center(&mut self, title: &str, area: Rect) {
        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        self.terminal.draw(|f|{
            f.render_widget(body_block, area)
        });
    }

    fn render_block_with_tab(&mut self, tabs: Vec<&str>, current: usize, area: Rect) {
        let tabs_spans: Vec<Span> = tabs
            .into_iter()
            .enumerate()
            .map(|(i, content)| {
                match i {
                    current if current > 0 => Span::styled(content, Style::default().fg(Color::LightYellow)),
                    0 => Span::from(content),
                    _ => Span::from(content),
                }
            })
            .collect();


        let body_block = Block::default()
            .borders(Borders::ALL)
            .title(tabs_spans)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);


        self.terminal.draw(|f|{
            f.render_widget(body_block, area)
        });
    }

    fn render_help_window<'a>(&mut self, doc_handler: &'a DocReaderHandler, area: Rect) {
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
        let popup_area = BackendTuiRs::centered_rect(60, 75, area);

        self.terminal.draw(|f| f.render_widget(Clear, popup_area));
        self.terminal.draw(|f| f.render_widget(popup_text, popup_area));
    }

    fn render_text<'a>(&mut self, text: &str, area: Rect) {
        let text = Paragraph::new(text)
            .alignment(Alignment::Left);

        self.terminal.draw(|f| f.render_widget(text, area));
    }

    fn render_text_in_block<'a>(&mut self, block_title: &str, text: &str, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(block_title)
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = Paragraph::new(text)
            .alignment(Alignment::Left)
            .block(block);

        self.terminal.draw(|f| f.render_widget(text, area));
        // self.terminal.draw(|f| f.render_widget(block, area));
    }

    // TODO: Add Custom Style
    fn render_text_with_bg<'a>(&mut self, text: &str, area: Rect) {
        let block = Paragraph::new(text)
            .style(Style::default().bg(Color::Blue).fg(Color::Black))
            .alignment(Alignment::Center);

        self.terminal.draw(|f| f.render_widget(block, area));
    }
}

impl Backend for BackendTuiRs {}