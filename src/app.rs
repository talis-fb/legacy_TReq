#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::hash_map::HashMap;

use tui::layout::Rect;
use tui::widgets::Widget;

pub enum METHODS {
    DELETE,
    GET,
    HEAD,
    OPTIONS,
    PATCH,
    POST,
    PUT,
}

pub struct Request {
    pub name: String,
    url: String,
    method: METHODS,
    headers: String,
    body: String,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            method: METHODS::GET,
            name: String::from("New Request"),
            url: String::new(),
            headers: String::new(),
            body: String::new(),
        }
    }
}

#[derive(Default)]
pub struct App {
    request_history: Vec<Request>,
    current_request: usize,
}

impl App {
    pub fn get_requests(&self) -> &Vec<Request> {
        &self.request_history
    }

    pub fn get_current_request(&self) -> &Request {
        &self.request_history[self.current_request]
    }

    pub fn create_request(&mut self, req: Request) {
        self.request_history.push(req);
    }
}

// STATE MANAGENT
