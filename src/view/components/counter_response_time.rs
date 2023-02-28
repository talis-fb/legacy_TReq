use super::Component;
use crate::view::renderer::Tui;
use crate::view::style::{Color, Style, Texts, Size};
use crate::view::{renderer::tui_rs::BackendTuiRs, style::Text};
use tui::layout::{Constraint, Layout, Rect};

pub struct CounterResponseTime {
    pub area: Rect,
    pub marked: bool,
    pub time: f64,
}
impl Component for CounterResponseTime {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let time_string = format!("{:.1}s", self.time);

        f.render_clear_area(self.area);

        if self.marked {
            f.render_block_with_title_center_marked(Texts::from_str("Requesting..."), self.area);
        } else {
            f.render_block_with_title_center(Texts::from_str("Requesting..."), self.area);
        }

        let area = Layout::default()
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(self.area);

        let content_area = BackendTuiRs::create_absolute_centered_area(Size::Percentage(100), Size::Fixed(1), area[0]);

        f.render_text_raw_align_center(&time_string, content_area);
    }
}
