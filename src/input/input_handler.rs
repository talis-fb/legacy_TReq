use crossterm::event::{self, Event};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use std::time::Duration;

use crate::app::InputMode;
use crate::base::actions::Actions;
use crate::base::os::file_edition_handler::FileEditionHandler;
use crate::config::configurations::external_editor::ExternalEditor;
use crate::utils::custom_types::uuid::UUID;

use tokio::task::JoinHandle;

use crate::input::keymaps::docs_mode;
use crate::input::keymaps::input_mode;
use crate::input::keymaps::normal_mode;

use super::listener::KeyboardListerner;
use std::process::{Command as OSCommand, Stdio};

pub struct InputHandler {
    // Save files
    configuration: Rc<ExternalEditor>,
    files: Rc<Mutex<FileEditionHandler>>,

    // Listener
    current_listener: Arc<Mutex<KeyboardListerner>>,
    listeners: HashMap<InputMode, KeyboardListerner>,
    last_input_mode_state: Option<InputMode>,

    // Send event
    sender_events: Sender<Actions>,

    // Task listener
    task_async_listener: Option<JoinHandle<()>>,
    finisher_async_listener: Option<Sender<()>>,
}
impl InputHandler {
    pub fn init(
        listener: KeyboardListerner,
        configuration: Rc<ExternalEditor>,
        files: Rc<Mutex<FileEditionHandler>>,
        sender_events: Sender<Actions>,
    ) -> Self {
        let listeners = HashMap::from([
            (
                InputMode::Normal,
                KeyboardListerner::init(normal_mode::keymap_factory()),
            ),
            (
                InputMode::Insert,
                KeyboardListerner::init(input_mode::keymap_factory()),
            ),
            (
                InputMode::Help,
                KeyboardListerner::init(docs_mode::keymap_factory()),
            ),
        ]);

        let current_listener = listeners.get(&InputMode::Normal);

        Self {
            current_listener: Arc::new(Mutex::new(listener)),
            configuration,
            files,
            last_input_mode_state: None,
            listeners,
            sender_events,

            task_async_listener: None,
            finisher_async_listener: None,
        }
    }
    pub fn close(&mut self) {
        let sender = self.finisher_async_listener.take();
        if sender.is_some() {
            sender.unwrap().send(()).unwrap();
        }

        let task = self.task_async_listener.take();
        if task.is_some() {
            task.unwrap().abort();
        }
    }

    pub fn update(&mut self, new_input_mode: InputMode) {
        let last_mode = self.last_input_mode_state;

        if last_mode.is_none() || last_mode.unwrap() != new_input_mode {
            // Update new mode
            self.last_input_mode_state = Some(new_input_mode);

            match new_input_mode {
                InputMode::Vim => {
                    self.close();
                }

                input_mode => {
                    if self.task_async_listener.is_none() && self.finisher_async_listener.is_none()
                    {
                        self.open_async_listener();
                    }

                    let listener = self.listeners.get(&input_mode).unwrap();
                    self.set_keymap(listener.clone());
                }
            }
        }
    }

    fn set_keymap(&mut self, keyboard_listener: KeyboardListerner) {
        let mut listener = self.current_listener.lock().unwrap();
        *listener = keyboard_listener;
    }

    fn open_async_listener(&mut self) {
        let listener = self.current_listener.clone();

        let (finished_sender, finished_listener): (Sender<()>, Receiver<()>) = mpsc::channel();

        let queue = self.sender_events.clone();

        let task = tokio::task::spawn(async move {
            let action_default = Actions::Null;

            while finished_listener.try_recv().is_err() {
                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let Event::Key(key) = event::read().unwrap() {
                        let mut keymap = listener.lock().unwrap();
                        let action = keymap.get_command(key.code).unwrap_or(action_default);

                        let res = queue.send(action);

                        if let Err(e) = res {
                            println!("Erro at run command: ...");
                            println!("{}", e);
                        }
                    }
                }
            }
        });

        self.task_async_listener = Some(task);
        self.finisher_async_listener = Some(finished_sender);
    }

    pub fn sync_open_vim(&mut self, buffer: String, uuid: &UUID) -> Result<String, String> {
        self.files
            .lock()
            .unwrap()
            .save_content(uuid, buffer)
            .unwrap();
        let file_path = self.files.lock().unwrap().get_path(uuid)?;

        let mut child = OSCommand::new(&self.configuration.editor)
            .arg(file_path.clone())
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to execute child");

        let status = child.wait().expect("failed to wait on child");

        std::fs::read_to_string(file_path).map_err(|e| e.to_string())
    }
}
