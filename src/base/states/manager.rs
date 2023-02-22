use std::rc::Rc;

use super::{
    names::StatesNames,
    states::{CommandsMap, State, StatesMap},
};

pub struct StateManager {
    states_map: StatesMap,

    // States
    current_state: Option<Rc<Box<dyn State>>>,
    default_state: Rc<Box<dyn State>>,

    // Always on
    // If some keys conflit with current_state, the global one needs to be ignored
    global_state: Rc<Box<dyn State>>,

    history_states: Vec<StatesNames>,
}
impl StateManager {
    pub fn init(default_state: impl State + 'static, global_state: impl State + 'static) -> Self {
        Self {
            default_state: Rc::new(Box::new(default_state)),
            global_state: Rc::new(Box::new(global_state)),
            current_state: None,
            history_states: vec![],

            states_map: StatesMap::init(),
        }
    }

    pub fn get_state(&self) -> &Box<dyn State> {
        self.current_state.as_ref().unwrap_or(&self.default_state)
    }

    pub fn set_state(&mut self, new_state: impl State + 'static) {
        self.set_state_name(new_state.get_state_name())
    }

    pub fn set_state_name(&mut self, new_state: StatesNames) {
        let current_state_name = self
            .current_state
            .as_ref()
            .unwrap_or(&self.default_state)
            .get_state_name();

        let last_state_name = self.history_states.last();

        if last_state_name.is_none() || *last_state_name.unwrap() != current_state_name {
            self.history_states.push(current_state_name);
        }

        let new_state = self.states_map.get(new_state);

        self.current_state = Some(new_state.unwrap());
    }

    pub fn set_state_default(&mut self) {
        self.current_state = None;
    }

    pub fn get_command_map(&self) -> CommandsMap {
        let main_map = self.get_state().get_map().clone();
        let mut global_map = self.global_state.get_map().clone();

        // With this, when find conflits of keys in both, main_map has priorite
        global_map.extend(main_map);

        global_map
    }

    pub fn reset_to_last_state(&mut self) -> StatesNames {
        let current_state = self
            .current_state
            .as_ref()
            .unwrap_or(&self.default_state)
            .get_state_name();

        let last_state = self
            .history_states
            .iter()
            .rev()
            .find(|state| **state != current_state)
            .unwrap();

        let state = *last_state;

        self.set_state_name(state);
        state
    }
}
