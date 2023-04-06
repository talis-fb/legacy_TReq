use std::collections::HashMap;

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

#[derive(Deserialize, Serialize)]
pub struct EnvironmentVars {
    pub global: Vec<String>,
    pub session: Vec<String>,
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

    pub fn sync(
        &mut self,
        global_variables: &HashMap<String, String>,
        session_variables: &HashMap<String, String>,
    ) {
        // Update SESSION variables
        self.vars_keys
            .session
            .retain(|key| session_variables.contains_key(key));

        let new_keys: Vec<String> = session_variables
            .keys()
            .filter(|key| !self.vars_keys.session.contains(key))
            .cloned()
            .collect();

        self.vars_keys.session.extend(new_keys);

        // Update GLOBAL variables
        self.vars_keys
            .global
            .retain(|key| global_variables.contains_key(key));

        let new_keys: Vec<String> = global_variables
            .keys()
            .filter(|key| !self.vars_keys.global.contains(key))
            .cloned()
            .collect();

        self.vars_keys.global.extend(new_keys);

        // Update INDEXES
        if self.vars_keys.session.get(self.current_session_var).is_none() {
            self.current_session_var = 0;
        }
        if self.vars_keys.global.get(self.current_global_var).is_none() {
            self.current_global_var = 0;
        }
    }
}
