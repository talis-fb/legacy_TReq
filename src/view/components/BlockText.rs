use super::Component;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::Texts;
use tui::layout::Rect;

pub struct BlockText {
    pub area: Rect,
    pub title: String,
    pub content: String,
}
impl Component for BlockText {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let title = Texts::from_str(&self.title);
        let content = Texts::from_str(&self.content);
        f.render_text_in_block(title, content, self.area);
    }
}

