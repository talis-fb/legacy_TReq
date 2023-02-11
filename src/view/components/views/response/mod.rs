use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::Texts;
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::{Constraint, Direction, Layout, Rect};


use self::response_content_view::{StatesResEditionView, ResposeEditionView};

pub mod response_status_view;
pub mod response_content_view;

pub struct ResponseView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
}
impl Component for ResponseView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let response_arc = self.store.get_response();
        let response = response_arc.lock().unwrap();

        let response_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
            .split(self.area);

        // All block area
        f.render_block_with_title_center(Texts::from_str("Response"), self.area);

        //
        // TODO: Color for this below
        //
        // Status Block
        f.render_text_with_bg(Texts::from_str("STATUS"), response_layout[0]);


        // Edition Block
        let edition_layout = response_layout[1];
        let headers_content = serde_json::to_string_pretty(&response.headers).unwrap_or_default();
        let edition_block = ResposeEditionView {
            area: edition_layout,
            body: &response.body,
            headers: &headers_content,
            opened: StatesResEditionView::BodyOpened
        };

        edition_block.render(f);
    }
}
