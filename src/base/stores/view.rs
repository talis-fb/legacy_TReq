use crate::view::views::environment::store::EnvironmentVars;
use crate::view::views::environment::store::OpenedVars;
use crate::view::views::environment::store::State as StateEnvironmentView;
use crate::view::views::request::store::State as StateRequestView;
use crate::view::views::request::store::StatesReqEditionView;
use crate::view::views::response::store::State as StateResponseView;
use crate::view::views::response::store::StatesResEditionView;

pub struct ViewStore {
    pub environment: StateEnvironmentView,
    pub request: StateRequestView,
    pub response: StateResponseView,
}

impl ViewStore {
    pub fn init() -> Self {
        Self {
            environment: StateEnvironmentView {
                opened_section: OpenedVars::Session,
                vars_keys: EnvironmentVars {
                    global: vec![],
                    session: vec![],
                },
                current_global_var: 0,
                current_session_var: 0,
            },
            request: StateRequestView {
                opened: StatesReqEditionView::BodyOpened,
            },
            response: StateResponseView {
                opened: StatesResEditionView::BodyOpened,
            },
        }
    }
}
