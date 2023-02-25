use std::sync::{Arc, Mutex, mpsc::{self, Sender, Receiver}};

use crate::app::App;

use super::Command;

pub struct CommandHandler {
    command_queue: Arc<Mutex<Vec<Command>>>,
}

impl CommandHandler {
    pub fn execute(app: &mut App, command: Command) -> Result<(), String> {
        command.execute(app)
    }

    pub fn init() -> Self {
        Self {
            command_queue: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn add(&mut self, command: Command) {
        let mut queue = self.command_queue.lock().unwrap();
        queue.push(command);
    }

    pub fn run(&mut self, app: &mut App) -> Result<(), String> {
        let mut queue = self.command_queue.lock().unwrap();
        let command_to_exec = queue.pop().unwrap();
        command_to_exec.execute(app)
    }
}
