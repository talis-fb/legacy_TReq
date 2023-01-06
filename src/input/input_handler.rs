use crossterm::event::{self, Event, KeyCode};
use std::{sync::{mpsc::Sender, Arc, Mutex}, io::Read};

use tempfile::Builder;

use crate::{
    app::app::App,
    base::{actions::Actions, store::DataStore},
    utils::AsyncBool,
};

use super::listener::KeyboardListerner;
use std::process::{Stdio, Command as OSCommand};
use std::io::Write;

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

    pub fn sync_open_vim(&self, buffer: String) -> (String, bool) {

        let temp_file = Builder::new()
            .suffix(".json")
            .tempfile()
            .unwrap();
        
        let mut file = temp_file.as_file();
        file.write_all(buffer.as_bytes()).unwrap();

        let file_path = temp_file.path();

        let mut child = OSCommand::new("nvim")
            .arg(file_path)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to execute child");

        let status = child.wait().expect("failed to wait on child");

        let content = std::fs::read_to_string(file_path).unwrap();

        (content, true)
    }
}
