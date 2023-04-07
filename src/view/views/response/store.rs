// use crate::view::style::Color;
use serde::{Deserialize, Serialize};

// Manage the State of view
#[derive(Deserialize, Serialize)]
pub struct State {
    // pub focus: bool,
    pub opened: StatesResEditionView,
    // pub status_color: Color,
    // pub status_text: String,
}

impl State {
    pub fn switch_opened(&mut self) {
        self.opened = match self.opened {
            StatesResEditionView::BodyOpened => StatesResEditionView::HeadersOpened,
            StatesResEditionView::HeadersOpened => StatesResEditionView::BodyOpened,
        }
    }

    pub fn open_body_view(&mut self) {
        self.opened = StatesResEditionView::BodyOpened;
    }

    pub fn open_headers_view(&mut self) {
        self.opened = StatesResEditionView::HeadersOpened;
    }
}

#[derive(Deserialize, Serialize)]
pub enum StatesResEditionView {
    BodyOpened,
    HeadersOpened,
}
