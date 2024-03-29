use crate::view::components::tab_blocked_text::TabBlockText;
use crate::view::components::Component;
use crate::view::renderer::tui_rs::BackendTuiRs;
use tui::layout::Rect;

use crate::view::views::response::store::StatesResEditionView;

pub struct ResposeEditionView<'a> {
    pub area: Rect,

    pub body: &'a str,
    pub headers: &'a str,
    pub opened: StatesResEditionView,
    pub marked: bool,
}
impl Component for ResposeEditionView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let mut block = TabBlockText {
            area: self.area,
            texts: vec![("Body", self.body), ("Headers", self.headers)],
            current: 0,
            marked: self.marked,
        };

        match self.opened {
            StatesResEditionView::BodyOpened => {
                block.texts[0].0 = "BODY";
                block.current = 0;
            }
            StatesResEditionView::HeadersOpened => {
                let _title_text = block.texts[1].0;
                block.texts[1].0 = "HEADERS";
                block.current = 1;
            }
        }

        block.render(f);
    }
}
