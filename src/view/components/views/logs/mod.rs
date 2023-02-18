use crate::base::logs::LogType;
use crate::view::ViewStates;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::{Color, Style, Text, Texts};
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::{Constraint, Direction, Layout, Rect};

pub mod log_view;

pub struct LogView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
    pub states: &'a ViewStates,
}

impl LogView<'_> {
    pub fn prepare_render<'b>(states: &mut ViewStates, store: &'b MainStore) {
        // states.insert("", v)
    }
}

impl Component for LogView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
            .split(self.area);

        let empty_string = String::new();

        let title = &self.store.log.title;
        let detail = self.store.log.detail.as_ref().unwrap_or(&empty_string);
        let log_type = &self.store.log.log_type;

        let color_title = match log_type {
            LogType::Error => Color::Red,
            LogType::Help => Color::Blue,
            LogType::Empty => Color::Black,
            LogType::Warning => Color::Yellow,
            LogType::InputMode => Color::Cyan,
        };

        let content = Texts {
            body: vec![
                Text {
                    body: title,
                    style: Some(Style {
                        color: color_title,
                        property: None,
                    }),
                },
                Text {
                    body: detail,
                    style: None,
                },
            ],
        };

        f.render_divider_with_text(Texts::from_str("Logs"), layout[0]);
        f.render_text(content, layout[1]);
    }
}
