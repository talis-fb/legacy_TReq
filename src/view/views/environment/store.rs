use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Var {
    pub key: String,
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
    pub vars_keys: EnvironmentVars,
}

impl State {
    pub fn get_current_var_key(&self) -> String {
        match self.opened_section {
            OpenedVars::Global => {
                let index = self.current_global_var;
                self.vars_keys.global.get(index).unwrap().clone()
            }

            OpenedVars::Session => {
                let index = self.current_session_var;
                self.vars_keys.session.get(index).unwrap().clone()
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct EnvironmentVars {
    pub global: Vec<String>,
    pub session: Vec<String>,
}

