use crate::base::commands::{Command, Commands};

use super::OsCommandTrait;
use tokio::sync::mpsc::{self, Sender};
use std::{
    path::PathBuf,
    process::{Command as OSCommand, Stdio},
};

pub type OsCommandEditor = Box<dyn OsCommandTrait>;

pub struct ExternalEditor {
    pub command_editor: String,
    pub path: PathBuf,
}

impl OsCommandTrait for ExternalEditor {
    fn exec(&self, sender: Sender<Command>) -> Result<(), String> {
        let mut child = OSCommand::new(&self.command_editor)
            .arg(self.path.clone())
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to execute Command");

        let _status = child.wait().expect("failed to wait Command");

        let content = std::fs::read_to_string(self.path.clone()).map_err(|e| e.to_string())?;

        // Commands::set


        Ok(())
    }

    // Spawn a thread to run process
    // fn async_open(&self) -> Result<mpsc::Receiver<String>, String> {
    //     let (sender, receiver) = mpsc::channel::<String>();
    //
    //     tokio::task::spawn({
    //         let path = self.path.clone();
    //         let command_editor = self.command_editor.clone();
    //         let self_clone = Self {
    //             command_editor,
    //             path,
    //         };
    //         async move {
    //             let output = self_clone.sync_open().unwrap();
    //             sender.send(output).unwrap();
    //         }
    //     });
    //
    //     Ok(receiver)
    // }

    // Todo: Make it verify if command set in EDITOR is valid
    // fn is_valid(&self) -> bool {
    //     std::env::var("EDITOR").is_ok()
    // }
}

impl ExternalEditor {
    pub fn init(path: PathBuf) -> Result<Self, String> {
        let command_editor = std::env::var("EDITOR").map_err(|e| e.to_string())?;
        Ok(Self {
            command_editor,
            path,
        })
    }
}
