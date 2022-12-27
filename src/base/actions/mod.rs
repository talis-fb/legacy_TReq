use std::collections::HashMap;

pub type EventListenerFn = fn();

pub mod manager;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Actions {
    Null,
    SubCommand, // When user press a key that has subcommands from it

    // Main actions
    Edit,
    Switch,
    Submit,
    Undo,
    New,

    // General Moves
    Up,
    Down,
    Left,
    Right,

    // Jumps to sections
    GoToTabList,
    GoToRequest,
    GoToResponse,
    GoToLogs,

    // Moves Tabs
    GoToNextTab,
    GoToPreviousTab,

    // Moves Request
    GoToRequestBody,
    GoToUrl,

    // Moves Response
    GoToResponseBody,
    GoToResponseHeaders,

    // Edit Tabs
    RenameTab,
    DeleteTab,

    // Edit Request
    RequestBodyEdit,
    RequestHeadersEdit,
    UrlEdit,
    MethodEdit,
}