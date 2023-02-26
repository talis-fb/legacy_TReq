use crossterm::event::{self, Event};
use std::rc::Rc;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};

use crate::base::actions::Actions;
use crate::base::os::file_edition_handler::FileEditionHandler;
use crate::config::configurations::external_editor::ExternalEditor;
use crate::utils::custom_types::uuid::UUID;

use super::listener::KeyboardListerner;
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

    pub fn async_handler_loop(
        &self,
        queue: Sender<Actions>,
    ) -> (tokio::task::JoinHandle<()>, Sender<()>) {
        let listener = self.listener.clone();

        let (finished_sender, finished_listener): (Sender<()>, Receiver<()>) = mpsc::channel();

        // TODO: Close this task when application shutdown
        let task = tokio::task::spawn(async move {
            let action_default = Actions::Null;

            // log::info!("-b READ");

            while finished_listener.try_recv().is_err() {
                if let Event::Key(key) = event::read().unwrap() {
                    // log::info!("-m READ");

                    let mut keymap = listener.lock().unwrap();
                    let action = keymap.get_command(key.code).unwrap_or(action_default);

                    let res = queue.send(action);
                    // log::info!("-send READ");

                    if let Err(e) = res {
                        println!("Erro at run command: ...");
                        println!("{}", e);
                    }
                }
            }

            // log::info!("ACABOUUUUUUUUUUU READ");
        });

        (task, finished_sender)
    }

    pub fn sync_open_vim(&mut self, buffer: String, uuid: &UUID) -> String {
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

        std::fs::read_to_string(file_path).unwrap()
    }
}
