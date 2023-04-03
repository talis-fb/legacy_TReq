// use crate::base::web::request::METHODS;
// use crate::view::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct State {
    pub opened: StatesReqEditionView,
    // pub method: METHODS,
    // pub color: Color,
    // pub url_block_focus: bool,
    // pub body_block_focus: bool,
}

impl State {
    pub fn switch_opened(&mut self) {
        self.opened = match self.opened {
            StatesReqEditionView::BodyOpened => StatesReqEditionView::HeadersOpened,
            StatesReqEditionView::HeadersOpened => StatesReqEditionView::BodyOpened,
        }
    }

    pub fn open_body_view(&mut self) {
        self.opened = StatesReqEditionView::BodyOpened;
    }

    pub fn open_headers_view(&mut self) {
        self.opened = StatesReqEditionView::HeadersOpened;
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum StatesReqEditionView {
    BodyOpened,
    HeadersOpened,
}
