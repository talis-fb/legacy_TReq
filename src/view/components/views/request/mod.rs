use std::collections::HashMap;

use crate::base::states::names::StatesNames;
use crate::base::web::request::METHODS;
use crate::view::ViewStates;
use crate::view::components::block_text::BlockText;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::{Color, Texts};
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::{Constraint, Direction, Layout, Rect};

pub mod request_edition_view;

use super::request::request_edition_view::{RequestEditionView, StatesReqEditionView};

pub struct RequestView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
    pub states: &'a ViewStates,
}

impl RequestView<'_> {
    pub fn prepare_render<'b>(states: &mut ViewStates, store: &'b MainStore) {
        // states.insert("", v)
    }
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
        f.render_block_with_title_center(Texts::from_str("Request"), self.area);

        // Url Block
        let url_layout = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints([Constraint::Length(7), Constraint::Min(1)].as_ref())
            .split(request_layout[0]);

        let color_button_method = match request.method {
            METHODS::GET => Color::Blue,
            METHODS::POST => Color::Green,
            METHODS::PUT => Color::White,
            METHODS::PATCH => Color::Magenta,
            METHODS::DELETE => Color::Red,
            METHODS::HEAD => Color::Yellow,
        };

        f.render_text_with_bg(
            Texts::from_str(request.method.to_string().as_str()),
            color_button_method,
            url_layout[0],
        );

        let url_block = BlockText {
            area: url_layout[1],
            title: Texts::from_str("URL"),
            content: Texts::from_str(&request.url),
            marked: self.store.current_state == StatesNames::Url,
        };
        url_block.render(f);

        //
        // f.render_text_in_block(
        //     Texts::from_str("URL"),
        //     Texts::from_str(&request.url),
        //     url_layout[1],
        // );

        // Edition Block
        let edition_layout = request_layout[1];
        let headers_content = serde_json::to_string_pretty(&request.headers).unwrap_or_default();
        let edition_block = RequestEditionView {
            area: edition_layout,
            body: &request.body,
            headers: &headers_content,
            opened: match self.store.current_state {
                StatesNames::RequestHeaders => StatesReqEditionView::HeadersOpened,
                _ => StatesReqEditionView::BodyOpened,
            },
            marked: match self.store.current_state {
                StatesNames::RequestBody | StatesNames::RequestHeaders => true,
                _ => false,
            },
        };

        edition_block.render(f);
    }
}
