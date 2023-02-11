use super::Component;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::Texts;
use tui::layout::Rect;

pub struct Tabslist {
    pub area: Rect,
    pub tabs: Vec<String>,
    pub current: usize,
    pub marked: bool
}
impl Component for Tabslist {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let tabs_str = self.tabs.iter().map(|f| Texts::from_str(f)).collect();

        if self.marked {
            f.render_tablist_marked(tabs_str, self.current, self.area)
        } else {
            f.render_tablist(tabs_str, self.current, self.area)
        }

    }
}
