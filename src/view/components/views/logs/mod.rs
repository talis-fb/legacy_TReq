use crate::base::logs::Log;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::Texts;
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::{Constraint, Direction, Layout, Rect};

pub mod log_view;

pub struct LogView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
}
impl Component for LogView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
            .split(self.area);

        let Log {
            title,
            detail,
            log_type,
        } = &self.store.log;

        let mut content = title.clone();
        content.push_str(detail.as_ref().unwrap_or(&String::new()).as_str());

        f.render_divider_with_text(Texts::from_str("Logs"), layout[0]);
        f.render_text(Texts::from_str(content.as_str()), layout[1]);
    }
}
