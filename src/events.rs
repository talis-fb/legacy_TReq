use std::collections::HashMap;

pub type EventListenerFn = fn();

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum EVENTS {
    Null,
    SubCommand, // When user press a key that has subcommands from it

    // Navigation
    Up,
    Down,
    Left,
    Right,
    Edit,
    Switch,
    GoToTabList,
    GoToNextTab,
    GoToPreviousTab,
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
