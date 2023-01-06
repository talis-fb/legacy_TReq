use crossterm::event::{self, Event, KeyCode};
use std::sync::{mpsc::Sender, Arc, Mutex};

use crate::{
    app::app::App,
    base::{actions::Actions, store::DataStore},
    utils::AsyncBool,
};

use super::listener::KeyboardListerner;

pub struct InputHandler {
    listener: Arc<Mutex<KeyboardListerner>>,
}
impl InputHandler {
    pub fn init(listener: KeyboardListerner /* , action_queue: Sender<Actions> */) -> Self {
        Self {
            listener: Arc::new(Mutex::new(listener)),
        }
    }

    pub fn async_handler(&self, queue: Sender<Actions>, when_finish: Arc<AsyncBool>) {
        let listener = self.listener.clone();

        let reading = tokio::task::spawn(async move {
            let mut keymap = listener.lock().unwrap();
            if let Event::Key(key) = event::read().unwrap() {
                let action = keymap.get_command(key.code);

                if let Some(act) = action {
                    queue.send(act);
                }
                when_finish.set(true);
            }
        });
    }

    pub fn sync_handler_typing(&self, buffer: String) -> (String, bool) {
        let mut is_finished = false;
        let mut new_buffer = buffer.clone();

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Enter => {
                    is_finished = true;
                }
                KeyCode::Backspace => {
                    new_buffer.pop();
                }
                KeyCode::Char(i) => {
                    new_buffer.push(i);
                }
                KeyCode::Esc => {
                    new_buffer.clear();
                    is_finished = true;
                }
                _ => {}
            }
        }

        (new_buffer, is_finished)
    }
}
