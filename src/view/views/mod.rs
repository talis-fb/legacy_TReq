use std::collections::HashMap;

pub mod app;
pub mod logs;
pub mod request;
pub mod response;
pub mod tabs_request;

pub type ViewStates = HashMap<String, String>;
