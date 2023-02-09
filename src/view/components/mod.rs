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
// Components
// ------------------------------------------------

pub struct TextView {
    pub area: Rect,
    pub title: String,
    pub content: String,
}
impl Component for TextView {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        f.render_text_in_block(&self.title, &self.content, self.area);
    }
}

pub struct Tabslist {
    pub area: Rect,
    pub tabs: Vec<String>,
    pub current: usize,
}
impl Component for Tabslist {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let tabs_str = self.tabs.iter().map(|f| f.as_str()).collect();
        f.render_tablist(tabs_str, self.current, self.area)
    }
}
