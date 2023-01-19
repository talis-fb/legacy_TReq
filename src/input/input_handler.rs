use crossterm::event::{self, Event, KeyCode};
use std::rc::Rc;
use std::sync::{mpsc::Sender, Arc, Mutex};

use tempfile::Builder;

use crate::config::configurations::external_editor::ExternalEditor;
use crate::{base::actions::Actions, utils::custom_types::async_bool::AsyncBool};

use super::{buffer::InputKeyboardBuffer, listener::KeyboardListerner};
use std::io::Write;
use std::process::{Command as OSCommand, Stdio};

pub struct InputHandler {
    listener: Arc<Mutex<KeyboardListerner>>,
    configuration: Rc<ExternalEditor>
}
impl InputHandler {
    pub fn init(listener: KeyboardListerner, configuration: Rc<ExternalEditor>) -> Self {
        Self {
            listener: Arc::new(Mutex::new(listener)),
            configuration,
        }
    }

    pub fn async_handler(&self, queue: Sender<Actions>, when_finish: Arc<AsyncBool>) {
        let listener = self.listener.clone();

        let reading = tokio::task::spawn(async move {
            let mut keymap = listener.lock().unwrap();
            if let Event::Key(key) = event::read().unwrap() {
                let action = keymap.get_command(key.code);

                if let Some(act) = action {
                    let res = queue.send(act);
                    if let Err(_) = res {
                        return;
                    }
                }
                when_finish.set(true);
            }
        });
    }
    pub fn sync_handler_doc_reading(&self, index_to_start: i32) -> (usize, bool) {
        let mut new_index = index_to_start;
        if let Event::Key(key) = event::read().unwrap() {
            new_index = match key.code {
                KeyCode::Char('k') | KeyCode::Up => index_to_start - 1,
                KeyCode::Char('j') | KeyCode::Down => index_to_start + 1,
                _ => return (0, true),
            }
        }

        if new_index < 0 {
            new_index = 0
        }

        ((new_index as usize), false)
    }

    pub fn sync_handler_typing(&self, buffer: &mut InputKeyboardBuffer) -> (String, bool) {
        let mut is_finished = false;
        let mut new_buffer = buffer.clone();

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Enter => {
                    is_finished = true;
                }
                KeyCode::Backspace => {
                    new_buffer.value.pop();
                }
                KeyCode::Char(i) => {
                    new_buffer.value.push(i);
                }
                KeyCode::Esc => {
                    new_buffer.reset_to_backup();
                    is_finished = true;
                }
                _ => {}
            }
        }

        (new_buffer.value, is_finished)
    }

    pub fn sync_open_vim(&self, buffer: String) -> (String, bool) {
        let temp_file = Builder::new().suffix(".json").tempfile().unwrap();

        let mut file = temp_file.as_file();
        file.write_all(buffer.as_bytes()).unwrap();

        let file_path = temp_file.path();

        let mut child = OSCommand::new(&self.configuration.editor)
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
