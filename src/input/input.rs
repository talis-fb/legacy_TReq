use std::sync::{mpsc::Sender, Arc, Mutex};
use crossterm::event::{self, Event, KeyCode};

use crate::{app::app::App, base::{store::DataStore, actions::Actions}, utils::AsyncBool};

use super::listener::KeyboardListerner;

#[derive(Clone)]
pub struct InputKeyboardBuffer {
    pub buffer: String,
    pub on_close: fn(&mut App, String),
}
impl InputKeyboardBuffer {
    pub fn init() -> Self {
        InputKeyboardBuffer {
            buffer: "".to_string(),
            on_close: |app: &mut App, s: String| {},
        }
    }
    pub fn append_char(&mut self, ch: char) {
        self.buffer.push(ch);
    }
    pub fn pop_char(&mut self) {
        self.buffer.pop();
    }
    pub fn set_callback(&mut self, callback: fn(&mut App, String)) {
        self.on_close = callback;
    }
    pub fn exec(&self, app: &mut App) {
        let callback = self.on_close;
        callback(app, self.buffer.clone())
    }
}

pub struct InputHandler {
    listener: Arc<Mutex<KeyboardListerner>>,
    action_queue: Sender<Actions>,
}
impl InputHandler {
    pub fn init(listener: KeyboardListerner, action_queue: Sender<Actions>) -> Self {
        Self {
            listener: Arc::new(Mutex::new(listener)),
            action_queue,
        }
    }

    pub fn handler(&self, when_finish: Arc<AsyncBool>) {
        let actions_queue = self.action_queue.clone();
        let listener = self.listener.clone();

        let reading = tokio::task::spawn(async move {
            let mut keymap = listener.lock().unwrap();
            if let Event::Key(key) = event::read().unwrap() {
                let action = keymap.get_command(key.code);

                if let Some(act) = action {
                    actions_queue.send(act);
                }
                when_finish.set(true);
            }
        });
    }
}
