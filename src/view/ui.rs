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

use super::components::views::{app::AppView, response::ResponseView};
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

        let app_view = AppView {
            area: screen_area,
            store: data_store,
        };

        app_view.render(&mut self.backend);

        self.backend.draw_all();
    }
}
