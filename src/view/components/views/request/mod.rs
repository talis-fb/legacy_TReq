use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::{Constraint, Direction, Layout, Rect};


pub mod request_edition_view;

use super::request::request_edition_view::{RequestEditionView, StatesReqEditionView};

pub struct RequestView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
}
impl Component for RequestView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let request = self.store.get_request();

        let request_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
            .split(self.area);

        // All block area
        f.render_block_with_title_center("Request", self.area);

        // Url Block
        let url_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Length(7), Constraint::Min(1)].as_ref())
            .split(request_layout[0]);

        //
        // TODO: Color in this block of METHODS
        //

        f.render_text_with_bg(&request.method.to_string(), url_layout[0]);
        f.render_text_in_block("URL", &request.url, url_layout[1]);

        // Edition Block
        let edition_layout = request_layout[1];
        let headers_content = serde_json::to_string_pretty(&request.headers).unwrap_or_default();
        let edition_block = RequestEditionView {
            area: edition_layout,
            body: &request.body,
            headers: &headers_content,
            opened: StatesReqEditionView::BodyOpened,
        };

        edition_block.render(f);
    }
}
