use std::collections::HashMap;

pub type EventListenerFn = fn();

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum EVENTS {
    Null,

    // Navigation
    Up,
    Down,
    Left,
    Right,
    Edit,
    Switch,
    GoToTabList,
    GoToRequestBody,
    GoToResponseBody,
    GoToUrl,

    // Request
    BodyResponseEdit,
    UrlEdit,
    MethodEdit,
    SubmitRequest,

    // Response
    BodyRequestEdit,
}

pub struct EventManager {
    listerners: HashMap<EVENTS, Vec<EventListenerFn>>,
}

impl EventManager {
    pub fn subscribe(&mut self, event_type: EVENTS, listener: EventListenerFn) {
        self.listerners.entry(event_type.clone()).or_default();
        self.listerners.get_mut(&event_type).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event_type: EVENTS, listener: EventListenerFn) {
        self.listerners
            .get_mut(&event_type)
            .unwrap()
            .retain(|&x| x != listener);
    }

    pub fn notify(&self, event_type: EVENTS) {
        let listeners = self.listerners.get(&event_type).unwrap();
        for listener in listeners {
            listener();
        }
    }
}
