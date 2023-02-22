use crossterm::event::{self, Event, KeyCode};
use std::rc::Rc;
use std::sync::{mpsc::Sender, Arc, Mutex};

use crate::base::os::file_edition_handler::FileEditionHandler;
use crate::config::configurations::external_editor::ExternalEditor;
use crate::utils::custom_types::uuid::UUID;
use crate::{base::actions::Actions, utils::custom_types::async_bool::AsyncBool};

use super::{buffer::InputKeyboardBuffer, listener::KeyboardListerner};
use std::process::{Command as OSCommand, Stdio};

pub struct InputHandler {
    listener: Arc<Mutex<KeyboardListerner>>,
    configuration: Rc<ExternalEditor>,
    files: Rc<Mutex<FileEditionHandler>>,
}
impl InputHandler {
    pub fn init(
        listener: KeyboardListerner,
        configuration: Rc<ExternalEditor>,
        files: Rc<Mutex<FileEditionHandler>>,
    ) -> Self {
        Self {
            listener: Arc::new(Mutex::new(listener)),
            configuration,
            files,
        }
    }

    pub fn set_keymap(&mut self, keyboard_listener: KeyboardListerner) {
        let mut listener = self.listener.lock().unwrap();
        *listener = keyboard_listener;
    }

    pub fn async_handler(
        &self,
        queue: Sender<Actions>,
        when_finish: Arc<AsyncBool>,
    ) -> tokio::task::JoinHandle<()> {
        let listener = self.listener.clone();

        tokio::task::spawn(async move {
            let mut keymap = listener.lock().unwrap();

            let action_default = Actions::Null;
            let mut action = action_default;

            if let Event::Key(key) = event::read().unwrap() {
                action = keymap.get_command(key.code).unwrap_or(action_default);
            }

            let res = queue.send(action);
            if let Err(e) = res {
                println!("Erro at run command: {:?}", e);
            }

            when_finish.set(true);
        })
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

    pub fn sync_open_vim(&mut self, buffer: String, uuid: &UUID) -> (String, bool) {
        self.files
            .lock()
            .unwrap()
            .save_content(uuid, buffer)
            .unwrap();
        let file_path = self.files.lock().unwrap().get_path(uuid);

        let mut child = OSCommand::new(&self.configuration.editor)
            .arg(file_path.clone())
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
