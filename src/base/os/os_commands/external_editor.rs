use super::OsCommandTrait;
use std::sync::mpsc;
use std::{
    path::PathBuf,
    process::{Command as OSCommand, Stdio},
};

pub type OsCommandEditor = Box<dyn OsCommandTrait<PathBuf, String>>;

pub struct ExternalEditor {
    pub command: String,
}

impl OsCommandTrait<PathBuf, String> for ExternalEditor {
    fn sync_open(&self, args: PathBuf) -> Result<String, String> {
        let file_path = args;

        let mut child = OSCommand::new(&self.command)
            .arg(file_path.clone())
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to execute Command");

        let _status = child.wait().expect("failed to wait Command");

        std::fs::read_to_string(file_path).map_err(|e| e.to_string())
    }

    // Spawn a thread to run process
    fn async_open(&self, args: PathBuf) -> Result<mpsc::Receiver<String>, String> {
        let (sender, receiver) = mpsc::channel::<String>();

        tokio::task::spawn({
            let self_clone = Self {
                command: self.command.clone(),
            };
            async move {
                let output = self_clone.sync_open(args).unwrap();
                sender.send(output).unwrap();
            }
        });

        Ok(receiver)
    }

    // Todo: Make it verify if command set in EDITOR is valid
    fn is_valid(&self) -> bool {
        std::env::var("EDITOR").is_ok()
    }

    fn init() -> Result<Self, String> {
        let command = std::env::var("EDITOR").map_err(|e| e.to_string())?;
        Ok(Self { command })
    }
}
