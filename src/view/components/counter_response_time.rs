use super::Component;
use crate::view::renderer::Tui;
use crate::view::style::{Color, Size, Style, Texts};
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

        let area_total = Layout::default()
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(self.area);

        let centered_area = BackendTuiRs::create_absolute_centered_area(
            Size::Percentage(100),
            Size::Fixed(2),
            area_total[0],
        );

        let content_area = Layout::default().constraints([
            Constraint::Length(1),
            Constraint::Length(1),
        ]).split(centered_area);

        f.render_text_raw_align_center(&time_string, content_area[0]);
        f.render_text_raw_align_center("Press [ESC] to cancel", content_area[1]);
    }
}
