use crossterm::event::{self, Event};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc::{self, Sender};

use crate::app::InputMode;
use crate::base::actions::Actions;

use crate::base::os::handler::FileHandler;

use tokio::task::JoinHandle;

use crate::input::keymaps::docs_mode;
use crate::input::keymaps::insert_mode;
use crate::input::keymaps::normal_mode;

use super::listener::KeyboardListerner;

#[mockall::automock]
pub trait InputHandler {
    fn update(&mut self, new_input_mode: InputMode);
    fn set_keymap(&mut self, keyboard_listener: KeyboardListerner);
    fn open_async_listener(&mut self);
    fn close_async_listener(&mut self);
}

pub struct InputDefaultHandler {
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

impl InputDefaultHandler {
    pub fn init(
        listener: KeyboardListerner,
        // external_editor: Rc<OsCommandEditor>,
        _files: Rc<Mutex<FileHandler>>,
        sender_events: Sender<Actions>,
    ) -> Self {
        let listeners = HashMap::from([
            (
                InputMode::Normal,
                KeyboardListerner::init(normal_mode::keymap_factory()),
            ),
            (
                InputMode::Insert,
                KeyboardListerner::init(insert_mode::keymap_factory()),
            ),
            (
                InputMode::Help,
                KeyboardListerner::init(docs_mode::keymap_factory()),
            ),
        ]);

        let _current_listener = listeners.get(&InputMode::Normal);

        Self {
            current_listener: Arc::new(Mutex::new(listener)),
            last_input_mode_state: None,
            listeners,
            sender_events,

            task_async_listener: None,
            finisher_async_listener: None,
        }
    }
}

impl InputHandler for InputDefaultHandler {
    fn update(&mut self, new_input_mode: InputMode) {
        if self.task_async_listener.is_none() && self.finisher_async_listener.is_none() {
            self.open_async_listener();
        }

        let last_mode = self.last_input_mode_state;

        if last_mode.is_none() || last_mode.unwrap() != new_input_mode {
            self.last_input_mode_state = Some(new_input_mode);

            let listener = self.listeners.get(&new_input_mode).unwrap();
            self.set_keymap(listener.clone());
        }
    }

    fn set_keymap(&mut self, keyboard_listener: KeyboardListerner) {
        let mut listener = self.current_listener.lock().unwrap();
        *listener = keyboard_listener;
    }

    fn open_async_listener(&mut self) {
        let listener = self.current_listener.clone();

        let (finished_sender, mut finished_listener) = mpsc::channel::<()>(32);

        let queue = self.sender_events.clone();

        let task = tokio::task::spawn(async move {
            let action_default = Actions::Null;

            while finished_listener.try_recv().is_err() {
                if event::poll(Duration::from_millis(100)).unwrap() {
                    if let Event::Key(key) = event::read().unwrap() {
                        let action;
                        {
                            let mut keymap = listener.lock().unwrap();
                            action = keymap.get_command(key.code).unwrap_or(action_default);
                        }

                        let res = queue.send(action).await;

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

    fn close_async_listener(&mut self) {
        let finisher_sender = self.finisher_async_listener.take();
        if let Some(s) = finisher_sender {
            tokio::task::spawn(async move {
                s.send(()).await.unwrap();
            });
        }

        let task = self.task_async_listener.take();
        if let Some(t) = task {
            t.abort();
        }
    }
}
