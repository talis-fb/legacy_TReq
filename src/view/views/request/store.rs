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

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum StatesReqEditionView {
    BodyOpened,
    HeadersOpened,
}
