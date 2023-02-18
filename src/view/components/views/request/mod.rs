use super::request::request_edition_view::{RequestEditionView, StatesReqEditionView};
use crate::base::states::names::StatesNames;
use crate::base::web::request::METHODS;
use crate::view::components::block_text::BlockText;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::{Color, Texts};
use crate::view::ViewStates;
use crate::{base::stores::MainStore, view::components::Component};
use serde::{Deserialize, Serialize};
use tui::layout::{Constraint, Direction, Layout, Rect};

pub mod request_edition_view;

// Manage the State of view
#[derive(Deserialize, Serialize)]
struct State {
    opened: StatesReqEditionView,
    method: METHODS,
    color: Color,
    url_block_focus: bool,
    body_block_focus: bool,
}

static KEY_STATE: &str = "request_view__state";
// ------------------------

pub struct RequestView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
    pub states: &'a ViewStates,
}

impl RequestView<'_> {
    pub fn prepare_render<'b>(states: &mut ViewStates, store: &'b MainStore) {
        let req = store.get_request();
        let state = State {
            method: req.method,
            color: match req.method {
                METHODS::GET => Color::Blue,
                METHODS::POST => Color::Green,
                METHODS::PUT => Color::White,
                METHODS::PATCH => Color::Magenta,
                METHODS::DELETE => Color::Red,
                METHODS::HEAD => Color::Yellow,
            },
            body_block_focus: match store.current_state {
                StatesNames::RequestBody | StatesNames::RequestHeaders => true,
                _ => false,
            },
            url_block_focus: store.current_state == StatesNames::Url,
            opened: match store.current_state {
                StatesNames::RequestHeaders => StatesReqEditionView::HeadersOpened,
                _ => StatesReqEditionView::BodyOpened,
            },
        };

        states.insert(
            KEY_STATE.to_string(),
            serde_json::to_string(&state).unwrap(),
        );
    }
}

impl Component for RequestView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let request = self.store.get_request();
        let state: State = serde_json::from_str(&self.states.get(KEY_STATE).unwrap()).unwrap();

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

        f.render_text_with_bg(
            Texts::from_str(request.method.to_string().as_str()),
            state.color,
            url_layout[0],
        );

        let url_block = BlockText {
            area: url_layout[1],
            title: Texts::from_str("URL"),
            content: Texts::from_str(&request.url),
            marked: state.url_block_focus,
        };
        url_block.render(f);

        // Edition Block
        let edition_layout = request_layout[1];
        let headers_content = serde_json::to_string_pretty(&request.headers).unwrap_or_default();
        let edition_block = RequestEditionView {
            area: edition_layout,
            body: &request.body,
            headers: &headers_content,
            opened: state.opened,
            marked: state.body_block_focus,
        };

        edition_block.render(f);
    }
}
