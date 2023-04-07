use crate::base::states::names::StatesNames;
use crate::view::components::tab_list::Tabslist;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::views::ViewStates;
use crate::{base::stores::MainStore, view::components::Component};
use tui::layout::Rect;

static KEY_STATE: &str = "request_view__";
fn state_key(key: &str) -> String {
    KEY_STATE.to_string() + key
}

pub struct TabRequestView<'a> {
    pub area: Rect,
    pub store: &'a MainStore,
    pub states: &'a ViewStates,
}

impl TabRequestView<'_> {
    pub fn prepare_render<'b>(states: &mut ViewStates, store: &'b MainStore) {
        // Does nothing for while
    }
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
                        name.push('*')
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
