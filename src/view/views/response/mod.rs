use self::response_content_view::ResposeEditionView;
use crate::base::states::names::StatesNames;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::{Color, Texts};
use crate::view::views::ViewStates;
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::{Constraint, Direction, Layout, Rect};

pub mod response_content_view;
pub mod response_status_view;
pub mod store;
use store::StatesResEditionView;

static KEY_STATE: &str = "response_view__state";
// ------------------------

pub struct ResponseView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
    pub states: &'a ViewStates,
}

impl ResponseView<'_> {
    pub fn prepare_render<'b>(states: &mut ViewStates, store: &'b MainStore) {
        // let response_arc = store.get_response();
        // let response = response_arc.lock().unwrap();
        //
        // let req = store.get_request();
        // let state = State {
        //     opened: match store.current_state {
        //         StatesNames::ResponseHeader => StatesResEditionView::HeadersOpened,
        //         _ => StatesResEditionView::BodyOpened,
        //     },
        //     focus: match store.current_state {
        //         StatesNames::ResponseBody | StatesNames::ResponseHeader => true,
        //         _ => false,
        //     },
        //     status_color: match response.status {
        //         0 => Color::Gray,
        //         77 => Color::Red, // A STATUS CODE INTERNAL TO INTERNAL ERROR
        //         100..=199 => Color::Gray,
        //         200..=299 => Color::Green,
        //         300..=399 => Color::Yellow,
        //         400..=499 => Color::Magenta,
        //         500..=599 => Color::Red,
        //         _ => Color::Cyan,
        //     },
        //     status_text: match response.status {
        //         0 => String::from("Hit ENTER to submit"),
        //         77 => String::from("Error"), // A STATUS CODE INTERNAL TO INTERNAL ERROR
        //         _ => response.status.to_string(),
        //     },
        // };
        //
        // states.insert(
        //     KEY_STATE.to_string(),
        //     serde_json::to_string(&state).unwrap(),
        // );
    }
}

impl Component for ResponseView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let response_arc = self.store.get_response();
        let response = response_arc.lock().unwrap();

        let status_text = match response.status {
            0 => String::from("Hit ENTER to submit"),
            77 => String::from("Error"), // A STATUS CODE INTERNAL TO INTERNAL ERROR
            _ => response.status.to_string(),
        };

        let status_color = match response.status {
            0 => Color::Gray,
            77 => Color::Red, // A STATUS CODE INTERNAL TO INTERNAL ERROR
            100..=199 => Color::Gray,
            200..=299 => Color::Green,
            300..=399 => Color::Yellow,
            400..=499 => Color::Magenta,
            500..=599 => Color::Red,
            _ => Color::Cyan,
        };
        let marked = match self.store.current_state {
            StatesNames::ResponseBody | StatesNames::ResponseHeader => true,
            _ => false,
        };

        let opened = match self.store.current_state {
            StatesNames::ResponseHeader => StatesResEditionView::HeadersOpened,
            _ => StatesResEditionView::BodyOpened,
        };

        let response_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
            .split(self.area);

        // All block area
        f.render_block_with_title_center(Texts::from_str("Response"), self.area);

        // Status Block
        f.render_text_with_bg(
            Texts::from_str(&status_text),
            status_color,
            response_layout[0],
        );

        // Edition Block
        let edition_layout = response_layout[1];
        let headers_content = serde_json::to_string_pretty(&response.headers).unwrap_or_default();
        let edition_block = ResposeEditionView {
            area: edition_layout,
            body: &response.body,
            headers: &headers_content,
            marked,
            opened,
        };

        edition_block.render(f);
    }
}
