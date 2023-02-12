use crate::{app::InputMode, base::stores::MainStore, config::configurations::view::ViewConfig};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use super::components::views::response::ResponseView;
use super::components::Component;

use super::renderer::tui_rs::BackendTuiRs;
use super::{
    components::{
        doc_reader::DocReader,
        input_block::InputTextBlock,
        views::{logs::LogView, request::RequestView, tabs_request::TabRequestView},
    },
    style::Size,
};

pub struct UI {
    backend: BackendTuiRs,
}

impl UI {
    pub fn init() -> Self {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap_or(());
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        // TODO:
        // Receive this from main.rs
        let backend = BackendTuiRs {
            terminal,
            configs: ViewConfig::init(),
            queue_render: vec![],
        };

        UI { backend }
    }

    pub fn close(&mut self) {
        disable_raw_mode().unwrap();
        execute!(
            self.backend.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        self.backend.terminal.show_cursor().unwrap();
    }

    pub fn render(&mut self, data_store: &MainStore) {
        self.backend.terminal.autoresize().unwrap();
        let screen_area = self.backend.terminal.get_frame().size();

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
            .split(screen_area);

        let sizes_layout = data_store.config.view.lock().unwrap();
        let (left, right) = sizes_layout.get_dimension_percentage();

        let content_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(left as u16),
                    Constraint::Percentage(right as u16),
                ]
                .as_ref(),
            )
            .split(full_screen_layout[1]);

        let tabb = TabRequestView {
            area: full_screen_layout[0],
            store: data_store,
        };

        let req_edit = RequestView {
            area: content_layout[0],
            store: data_store,
        };

        let res_edit = ResponseView {
            area: content_layout[1],
            store: data_store,
        };

        let log_view = LogView {
            area: full_screen_layout[2],
            store: data_store,
        };

        let popup_component: Option<Box<dyn Component<Backend = BackendTuiRs>>> =
            match data_store.get_mode() {
                InputMode::Insert => Some(Box::new(InputTextBlock {
                    area: BackendTuiRs::create_absolute_centered_area(
                        Size::Percentage(60),
                        Size::Fixed(3),
                        screen_area,
                    ),
                    text: &data_store.input_buffer.value,
                })),
                InputMode::Help => Some(Box::new(DocReader {
                    area: BackendTuiRs::create_absolute_centered_area(
                        Size::Percentage(60),
                        Size::Percentage(75),
                        screen_area,
                    ),
                    doc_handler: data_store.doc_reader.as_ref().unwrap(),
                })),
                _ => None,
            };

        tabb.render(&mut self.backend);
        req_edit.render(&mut self.backend);
        res_edit.render(&mut self.backend);
        log_view.render(&mut self.backend);

        if let Some(b) = popup_component {
            b.render(&mut self.backend);
        }

        self.backend.draw_all();
    }
}
