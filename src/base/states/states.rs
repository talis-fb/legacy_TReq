use crate::actions::Actions;
use crate::commands::Command;
use std::collections::HashMap;
use super::names::StatesNames;

// Interfaces
pub type CommandsMap = HashMap<Actions, Command>;

pub trait State {
    fn get_map(&self) -> &CommandsMap;
    fn get_state_name(&self) -> StatesNames;
    fn init() -> Self
    where
        Self: Sized;
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
