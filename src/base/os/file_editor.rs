use std::path::PathBuf;
use std::process::{Command as OSCommand, Stdio};
use std::sync::mpsc;

#[mockall::automock]
pub trait OsCommand<Arg: 'static, T: 'static> {
    // Block current thread until return response of command
    fn sync_open(&self, args: Arg) -> Result<T, String>;

    // Spawn a thread to run process asynchronous
    fn async_open(&self, args: Arg) -> Result<mpsc::Receiver<T>, String>;

    fn is_valid(&self) -> bool;

    fn init() -> Result<Self, String>
    where
        Self: Sized;
}

pub type OsCommandEditor = Box<dyn OsCommand<PathBuf, String>>;

pub struct ExternalEditor {
    pub command: String,
}

impl OsCommand<PathBuf, String> for ExternalEditor {
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
            let command = self.command.clone();
            let file_path = args;
            async move {
                let mut child = OSCommand::new(command)
                    .arg(file_path.clone())
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("failed to execute Command");

                let _status = child.wait().expect("failed to wait Command");

                let response = std::fs::read_to_string(file_path)
                    .map_err(|e| e.to_string())
                    .unwrap();

                sender.send(response).unwrap();
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
