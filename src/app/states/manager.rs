use super::{CommandsMap, State};

pub struct StateManager {
    // States
    current_state: Option<Box<dyn State>>,
    default_state: Box<dyn State>,

    // Always on
    // TODO: if some keys conflit with current_state, the global one needs to be ignored
    global_state: Box<dyn State>,
}
impl StateManager {
    pub fn init(default_state: impl State + 'static, global_state: impl State + 'static) -> Self {
        Self {
            default_state: Box::new(default_state),
            global_state: Box::new(global_state),
            current_state: None,
        }
    }

    pub fn get_state(&self) -> &Box<dyn State> {
        self.current_state
            .as_ref()
            .map_or(&self.default_state, |s| s)
    }

    pub fn set_state(&mut self, new_state: impl State + 'static) {
        self.current_state = Some(Box::new(new_state));
    }

    pub fn set_state_default(&mut self) {
        self.current_state = None;
    }

    pub fn get_command_map(&self) -> CommandsMap {
        let main_state = self.get_state();
        main_state.get_map().clone()

        // let global_state = self.global_state.get_map().clone();

        // global_state.extend(main_state.into_iter().filter(|(k, v)| { !global_state.contains_key(k) }));
        // let aa = main_state.into_iter().map(|f| {
        // })
        // main_state.extend(global_state.iter())
    }
}
