use super::Component;
use crate::view::{renderer::tui_rs::BackendTuiRs, style::Size};
use crate::view::renderer::Tui;
use crate::view::style::Texts;
use tui::layout::{Constraint, Layout, Rect};

pub struct InputTextBlock<'a> {
    pub area: Rect,
    pub text: &'a str,
}
impl Component for InputTextBlock<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        f.render_clear_area(self.area);
        f.render_block_with_title_left(
            Texts::from_str("[ESC] - QUIT     [ENTER] - FINISH"),
            self.area
        );

        let layout_content = Layout::default()
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(self.area)[0];
        f.render_text_raw(&self.text.to_string(), layout_content);
    }
}
