use super::request::request_edition_view::RequestEditionView;
use super::request::store::{State, StatesReqEditionView};
use crate::base::states::names::StatesNames;
use crate::base::web::request::METHODS;
use crate::view::components::block_text::BlockText;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::{Color, Texts};
use crate::view::views::ViewStates;
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::{Constraint, Direction, Layout, Rect};

pub mod request_edition_view;
pub mod store;

// static KEY_STATE: &str = "request_view__state";
// ------------------------

pub struct RequestView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
    pub states: &'a ViewStates,
}

impl RequestView<'_> {
    pub fn prepare_render<'b>(states: &mut ViewStates, store: &'b MainStore) {
        // let state_json = states.entry(KEY_STATE.to_string()).or_default();
        // let last_state: Result<State, serde_json::Error> = serde_json::from_str(state_json);
        //
        // let req = store.get_request();
        // let state = State {
        //     method: req.method,
        //     color: match req.method {
        //         METHODS::GET => Color::Blue,
        //         METHODS::POST => Color::Green,
        //         METHODS::PUT => Color::White,
        //         METHODS::PATCH => Color::Magenta,
        //         METHODS::DELETE => Color::Red,
        //         METHODS::HEAD => Color::Yellow,
        //     },
        //     body_block_focus: match store.current_state {
        //         StatesNames::RequestBody | StatesNames::RequestHeaders => true,
        //         _ => false,
        //     },
        //     url_block_focus: store.current_state == StatesNames::Url,
        //     opened: match store.current_state {
        //         StatesNames::RequestHeaders => StatesReqEditionView::HeadersOpened,
        //         StatesNames::RequestBody => StatesReqEditionView::BodyOpened,
        //         _ => {
        //             if let Ok(s) = last_state {
        //                 s.opened
        //             } else {
        //                 StatesReqEditionView::BodyOpened
        //             }
        //         }
        //     },
        // };
        //
        // states.insert(
        //     KEY_STATE.to_string(),
        //     serde_json::to_string(&state).unwrap(),
        // );
    }
}

impl Component for RequestView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let request = self.store.get_request();
        // let state: State = serde_json::from_str(self.states.get(KEY_STATE).unwrap()).unwrap();
        // ----------------

        let req = self.store.get_request();

        let state: State = self.store.view.request;

        let state_color = match req.method {
            METHODS::GET => Color::Blue,
            METHODS::POST => Color::Green,
            METHODS::PUT => Color::White,
            METHODS::PATCH => Color::Magenta,
            METHODS::DELETE => Color::Red,
            METHODS::HEAD => Color::Yellow,
        };

        let body_block_focus = match self.store.current_state {
            StatesNames::RequestBody | StatesNames::RequestHeaders => true,
            _ => false,
        };

        let url_block_focus = self.store.current_state == StatesNames::Url;

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
            state_color,
            url_layout[0],
        );

        let url_block = BlockText {
            area: url_layout[1],
            title: Texts::from_str("URL"),
            content: Texts::from_str(&request.url),
            marked: url_block_focus,
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
            marked: body_block_focus,
        };

        edition_block.render(f);
    }
}
