use super::Component;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::Texts;
use tui::layout::{Rect, Layout, Constraint};

pub struct BlockText<'a> {
    pub area: Rect,
    pub title: Texts<'a>,
    pub content: Texts<'a>,
}
impl Component for BlockText<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let content_text_layout = Layout::default()
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(self.area);

        // TODO:
        // Find a better way to not clone here
        // It's not so bad because Texts only store references and enums
        // Thus, the clone it not so heavy. But it's still a Vec
        f.render_block_with_title_left(self.title.clone(), self.area);
        f.render_text(self.content.clone(), content_text_layout[0]);
    }
}

