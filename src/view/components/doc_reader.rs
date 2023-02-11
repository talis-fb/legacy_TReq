use super::Component;
use crate::base::doc::handler::DocReaderHandler;
use crate::view::{renderer::tui_rs::BackendTuiRs, style::Size};
use crate::view::renderer::Tui;
use crate::view::style::Texts;
use tui::layout::{Constraint, Layout, Rect};

pub struct DocReader<'a> {
    pub area: Rect,
    pub doc_handler: &'a DocReaderHandler,
}
impl Component for DocReader<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let texts = self.doc_handler.get_texts();

        let layout = Layout::default()
            .margin(1)
            .constraints([Constraint::Percentage(100)])
            .split(self.area)[0];

        f.render_clear_area(self.area);

        f.render_block_with_title_center(
            Texts::from_str("Navigate -> [UP] and [DOWN] / Press any other key to close"),
            self.area,
        );

        f.render_rows_texts(texts, layout);
    }
}
