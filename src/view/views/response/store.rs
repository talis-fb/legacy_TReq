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

#[derive(Deserialize, Serialize)]
pub enum StatesResEditionView {
    BodyOpened,
    HeadersOpened,
}
