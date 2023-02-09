use super::Component;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use tui::layout::Rect;

pub struct BlockText {
    pub area: Rect,
    pub title: String,
    pub content: String,
}
impl Component for BlockText {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        f.render_text_in_block(&self.title, &self.content, self.area);
    }
}