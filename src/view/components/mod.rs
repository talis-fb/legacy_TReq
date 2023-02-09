use tui::layout::Rect;
use crate::view::renderer::{Backend, Tui};
use super::renderer::tui_rs::BackendTuiRs;

pub trait Component {
    type Backend: Backend;
    fn render(&self, f: &mut Self::Backend);
}

pub trait StatedComponents<State> {
    fn render(&self, state: State, f: &mut impl Backend);
}

// ------------------------------------------------
// Components Implementations
// ------------------------------------------------
pub mod TabList;
pub mod BlockText;
pub mod TabBlockedText;