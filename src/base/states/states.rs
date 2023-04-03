use super::names::StatesNames;
use crate::actions::Actions;
use crate::commands::Command;
use std::collections::HashMap;
use std::rc::Rc;

// Interfaces
pub type CommandsMap = HashMap<Actions, Command>;

pub trait State {
    fn get_map(&self) -> &CommandsMap;
    fn get_state_name(&self) -> StatesNames;
    fn init() -> Self
    where
        Self: Sized;
}

pub struct StatesMap {
    states: HashMap<StatesNames, Rc<Box<dyn State>>>,
}
impl StatesMap {
    pub fn init() -> Self {
        let all_states: [Rc<Box<dyn State>>; 13] = [
            Rc::new(Box::new(LogsState::init())),
            Rc::new(Box::new(RequestActiveState::init())),
            Rc::new(Box::new(RequestHeaderActiveState::init())),
            Rc::new(Box::new(RequestUrlActiveState::init())),
            Rc::new(Box::new(ResponseBodyActiveState::init())),
            Rc::new(Box::new(ResponseHeadersState::init())),
            Rc::new(Box::new(TabActiveState::init())),
            Rc::new(Box::new(DefaultState::init())),
            Rc::new(Box::new(EmptyState::init())),
            Rc::new(Box::new(DefaultEditMode::init())),
            Rc::new(Box::new(DefaultHelpMode::init())),
            Rc::new(Box::new(EditingGlobalEnvState::init())),
            Rc::new(Box::new(EditingSessionEnvState::init())),
        ];

        let states: HashMap<StatesNames, Rc<Box<dyn State>>> = all_states
            .into_iter()
            .map(|value| {
                let key = value.get_state_name();
                (key, value)
            })
            .collect();

        Self { states }
    }

    pub fn get(&self, state: StatesNames) -> Option<Rc<Box<dyn State>>> {
        Some(self.states.get(&state)?.clone())
    }
}

// List of all States ------------------
mod active_logs;
pub use active_logs::LogsState;

mod active_request_body;
pub use active_request_body::RequestActiveState;

mod active_request_headers;
pub use active_request_headers::RequestHeaderActiveState;

mod active_request_url;
pub use active_request_url::RequestUrlActiveState;

mod active_response_body;
pub use active_response_body::ResponseBodyActiveState;

mod active_response_headers;
pub use active_response_headers::ResponseHeadersState;

mod active_tablist;
pub use active_tablist::TabActiveState;

mod default;
pub use default::DefaultState;

mod empty;
pub use empty::EmptyState;

mod default_edit_mode;
pub use default_edit_mode::DefaultEditMode;

mod default_help_mode;
pub use default_help_mode::DefaultHelpMode;

mod editing_global_env;
pub use editing_global_env::EditingGlobalEnvState;

mod editing_session_env;
pub use editing_session_env::EditingSessionEnvState;
