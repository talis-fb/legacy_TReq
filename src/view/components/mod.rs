use crate::view::renderer::Backend;

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
pub mod views;
