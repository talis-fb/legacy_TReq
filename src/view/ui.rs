use crate::{base::stores::MainStore, config::configurations::view::ViewConfig};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{collections::HashMap, io};
use tui::{backend::CrosstermBackend, Terminal};

use super::components::Component;
use super::{components::views::app::AppView, ViewStates};

use super::renderer::tui_rs::BackendTuiRs;

pub struct UI {
    backend: BackendTuiRs,
    view_states: ViewStates,
}

impl UI {
    pub fn init() -> Self {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap_or(());
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        let backend = BackendTuiRs {
            terminal,
            configs: ViewConfig::init(),
            queue_render: vec![],
        };

        UI {
            backend,
            view_states: HashMap::new(),
        }
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

        AppView::prepare_render(&mut self.view_states, data_store);

        let app_view = AppView {
            area: screen_area,
            store: data_store,
            states: &self.view_states,
        };

        app_view.render(&mut self.backend);

        self.backend.draw_all();
    }
}
