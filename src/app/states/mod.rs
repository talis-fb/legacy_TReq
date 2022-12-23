use crate::actions::Actions;
use crate::app::app::App;
use crate::commands::{self, Command, Commands};
use std::collections::HashMap;

// States Names
pub mod states_names;
pub use states_names::*;

pub mod manager;

pub type CommandsMap = HashMap<Actions, Command>;

pub trait State {
    fn get_map(&self) -> &CommandsMap;
    fn get_state_name(&self) -> StatesNames;
    fn init() -> Self
    where
        Self: Sized;
}

// List of all States ------------------
pub mod active_logs;
pub mod active_request_body;
pub mod active_request_headers;
pub mod active_request_url;
pub mod active_response_body;
pub mod active_response_headers;
pub mod active_tablist;
pub mod default;
pub mod empty;

pub mod States {
    use super::*;
    pub use active_logs::LogsState;
    pub use active_request_body::RequestActiveState;
    pub use active_request_headers::RequestHeaderActiveState;
    pub use active_request_url::RequestUrlActiveState;
    pub use active_response_body::ResponseBodyActiveState;
    pub use active_response_headers::ResponseHeadersState;
    pub use active_tablist::TabActiveState;
    pub use default::DefaultState;
    pub use empty::EmptyState;
}
