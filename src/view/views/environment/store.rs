use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Var {
    pub key: String,
}

#[derive(Deserialize, Serialize)]
pub struct EnvironmentVars {
    pub global: Vec<Var>,
    pub session: Vec<Var>,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum OpenedVars {
    Global,
    Session,
}

// Manage the State of view
#[derive(Deserialize, Serialize)]
pub struct State {
    pub opened_section: OpenedVars,
    pub current_global_var: usize,
    pub current_session_var: usize,
    pub vars: EnvironmentVars,
}
