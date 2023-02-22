use std::rc::Rc;

use super::{
    names::StatesNames,
    states::{CommandsMap, DefaultState, State, StatesMap},
};

pub struct StateManager {
    states_map: StatesMap,

    // States
    current_state: Option<Rc<Box<dyn State>>>,
    default_state: Rc<Box<dyn State>>,

    // Always on
    // If some keys conflit with current_state, the global one needs to be ignored
    global_state: Rc<Box<dyn State>>,

    last_state: Option<Rc<Box<dyn State>>>,
}
impl StateManager {
    pub fn init(default_state: impl State + 'static, global_state: impl State + 'static) -> Self {
        Self {
            default_state: Rc::new(Box::new(default_state)),
            global_state: Rc::new(Box::new(global_state)),
            current_state: None,
            last_state: None,

            states_map: StatesMap::init(),
        }
    }

    pub fn get_state(&self) -> &Box<dyn State> {
        self.current_state.as_ref().unwrap_or(&self.default_state)
    }

    pub fn set_state(&mut self, new_state: impl State + 'static) {
        self.last_state = Some(
            self.current_state
                .as_ref()
                .unwrap_or(&self.default_state)
                .clone(),
        );

        self.current_state = Some(Rc::new(Box::new(new_state)));
    }

    pub fn set_state_name(&mut self, new_state: StatesNames) {
        self.last_state = Some(
            self.current_state
                .as_ref()
                .unwrap_or(&self.default_state)
                .clone(),
        );
        self.current_state = self.states_map.get(new_state);
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
}
