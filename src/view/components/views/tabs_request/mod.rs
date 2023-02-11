use crate::base::states::names::StatesNames;
use crate::view::components::TabList::Tabslist;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::Rect;

pub struct TabRequestView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
}
impl Component for TabRequestView<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let request = self.store.get_requests();

        let component = Tabslist {
            area: self.area,
            tabs: request
                .iter()
                .map(|f| {
                    let mut name = f.name.clone();

                    if f.has_changed {
                        name.push_str("*")
                    }

                    name
                })
                .collect(),
            current: self.store.request_ind(),
            marked: self.store.current_state == StatesNames::TabList,
        };

        component.render(f)
    }
}
