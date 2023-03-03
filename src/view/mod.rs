use std::collections::HashMap;

pub mod components;
pub mod help;
pub mod renderer;
pub mod style;
pub mod ui;

pub type ViewStates = HashMap<String, String>;
