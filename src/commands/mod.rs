use crate::{app::App, request::Request};
use std::collections::HashMap;

type CommandFunc = fn(app: &mut App) -> Result<(), String>;

pub struct CommandsList {}
impl CommandsList {
    pub fn go_to_next_tab() -> CommandFunc {
        |app: &mut App| {
            let mut i = app.current_request.clone();
            let size_tabs = app.get_requests().len();
            i += 1;
            if i >= size_tabs {
                i = 0;
            }
            app.current_request += i;
            Ok(())
        }
    }
    pub fn add_new_tab() -> CommandFunc {
        |app: &mut App| {
            let req = Request::default();
            app.create_request(req);
            Ok(())
        }
    }
}

// ------------------
// OLD IMPLEMENTATIOV
// ------------------
pub trait Command {
    fn execute(&self, app: &mut App) -> Result<(), &str>;
}

#[derive(Default)]
pub struct GoToTabList {}
impl Command for GoToTabList {
    fn execute(&self, app: &mut App) -> Result<(), &str> {
        Ok(())
    }
}

#[derive(Default)]
pub struct GoToNextTab {}
impl Command for GoToNextTab {
    fn execute(&self, app: &mut App) -> Result<(), &str> {
        let mut i = app.current_request.clone();
        let size_tabs = app.get_requests().len();
        i += 1;
        if i >= size_tabs {
            i = 0;
        }
        app.current_request += i;
        Ok(())
    }
}

pub struct NewTab;
impl Command for NewTab {
    fn execute(&self, app: &mut App) -> Result<(), &str> {
        let req = Request::default();
        app.create_request(req);
        Ok(())
    }
}
