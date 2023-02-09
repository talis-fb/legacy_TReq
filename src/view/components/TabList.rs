use super::Component;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use tui::layout::Rect;

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
