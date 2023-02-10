use crate::{
    app::InputMode, base::states::names::StatesNames, base::stores::MainStore,
    config::configurations::view::ViewConfig,
};

use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Terminal,
};

use crate::view::drawers;

use super::components::views::request::RequestView;
use super::components::Component;
use super::components::TabList::Tabslist;
use super::renderer::tui_rs::BackendTuiRs;

// OLD -------------------------------------------------------------
pub struct UI {
    backend: BackendTuiRs,
    // terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    // configs: ViewConfig
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
        let fff = self.backend.terminal.get_frame();
        fff.size();

        // self.backend.terminal.draw(|f| rr = Some(f.size())).unwrap();

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
            .split(fff.size());

        let tabb = Tabslist {
            area: full_screen_layout[0],
            tabs: data_store
                .get_requests()
                .iter()
                .map(|f| f.name.clone())
                .collect(),
            current: data_store.request_ind(),
        };

        let req_edit = RequestView {
            area: full_screen_layout[1],
            store: data_store,
        };

        tabb.render(&mut self.backend);
        req_edit.render(&mut self.backend);

        self.backend.draw_all();

        // self.backend
        //     .terminal
        //     .draw(|f| {
        //         // tabb.render(&mut self.backend);
        //
        //         let current_state = data_store.current_state;
        //         let style_if_state_is = |state: StatesNames| {
        //             if state == current_state {
        //                 Style::default().fg(Color::LightYellow)
        //             } else {
        //                 Style::default()
        //             }
        //         };
        //
        //         let size = f.size();
        //
        //         let full_screen_layout = Layout::default()
        //             .direction(Direction::Vertical)
        //             .margin(0)
        //             .constraints(
        //                 [
        //                     // Request List Tab
        //                     Constraint::Length(3),
        //                     Constraint::Min(1),
        //                     Constraint::Length(2),
        //                 ]
        //                 .as_ref(),
        //             )
        //             .split(f.size());
        //
        //         // Layout request + response
        //         let sizes_layout = data_store.config.view.lock().unwrap();
        //         let (left, right) = sizes_layout.get_dimension_percentage();
        //         let content_layout = Layout::default()
        //             .direction(Direction::Horizontal)
        //             .margin(0)
        //             .constraints(
        //                 [
        //                     Constraint::Percentage(left as u16),
        //                     Constraint::Percentage(right as u16),
        //                 ]
        //                 .as_ref(),
        //             )
        //             .split(full_screen_layout[1]);
        //
        //         // REQUEST BLOCK
        //         let request_block = Block::default()
        //             .borders(Borders::ALL)
        //             .title("Request")
        //             .title_alignment(Alignment::Center)
        //             .border_type(BorderType::Rounded);
        //         f.render_widget(request_block, content_layout[0]);
        //
        //         let request_layout = Layout::default()
        //             .direction(Direction::Vertical)
        //             .margin(1)
        //             .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        //             .split(content_layout[0]);
        //
        //         // Tablit
        //         // drawers::draw_tablist_requests(f, full_screen_layout[0], data_store);
        //
        //         // Request
        //         drawers::draw_method_and_url(f, request_layout[0], data_store);
        //         drawers::draw_body_request_section(f, request_layout[1], data_store);
        //
        //         // Response
        //         drawers::draw_body_response_section(f, content_layout[1], data_store);
        //
        //         // Logs
        //         drawers::draw_logs_section(f, full_screen_layout[2], data_store);
        //
        //         // Variants of InputModes
        //         match data_store.get_mode() {
        //             InputMode::Insert => drawers::draw_input_popup(f, f.size(), data_store),
        //             InputMode::Help => drawers::draw_help_popup(f, f.size(), data_store),
        //             _ => {}
        //         }
        //     })
        //     .unwrap();
    }
}
