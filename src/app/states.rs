use crate::actions::Actions;
use crate::commands::{Command, Commands};
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
pub use active_logs::LogsState;

pub mod active_request_body;
pub use active_request_body::RequestActiveState;

pub mod active_request_headers;
pub use active_request_headers::RequestHeaderActiveState;

pub mod active_request_url;
pub use active_request_url::RequestUrlActiveState;

pub mod active_response_body;
pub use active_response_body::ResponseBodyActiveState;

pub mod active_response_headers;
pub use active_response_headers::ResponseHeadersState;

pub mod active_tablist;
pub use active_tablist::TabActiveState;

pub mod default;
pub use default::DefaultState;

pub mod empty;
pub use empty::EmptyState;
