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
pub mod block_text;
pub mod counter_response_time;
pub mod doc_reader;
pub mod input_insert_mode;
pub mod tab_blocked_text;
pub mod tab_list;
pub mod welcome_doc;
