use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

use crate::{app::App, base::commands::CommandType};

use super::{Command, CommandTrait};

pub struct CommandHandler {
    running_jobs: Arc<Mutex<HashMap<String, tokio::sync::mpsc::Sender<()>>>>,

    sender_commands: Sender<Command>,
    listener_commands: Receiver<Command>,
}

impl CommandHandler {
    pub fn execute(app: &mut App, command: Command) -> Result<(), String> {
        command.execute(app)
    }

    pub fn init() -> Self {
        let (tx, rx): (Sender<Command>, Receiver<Command>) = mpsc::channel();

        Self {
            sender_commands: tx,
            listener_commands: rx,
            running_jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add(&mut self, command: Command) {
        log::info!("Added command to queue");
        self.sender_commands.send(command).unwrap();
    }

    pub fn run(&mut self, app: &mut App) -> Result<(), String> {
        while let Ok(command_to_exec) = self.listener_commands.try_recv() {
            match command_to_exec.type_running() {
                CommandType::Sync => {
                    command_to_exec.execute(app).unwrap();
                }
                CommandType::Async => {
                    let key = command_to_exec.get_id();

                    let running_jobs_arc = self.running_jobs.clone();

                    // Verify if job of asyc commad is already proccessig
                    {
                        let running_jobs = running_jobs_arc.lock().unwrap();
                        if running_jobs.get(&key).is_some() {
                            return Err(String::from(key + " is already runnig"));
                        }
                    }

                    command_to_exec.execute(app).unwrap();

                    // Insert in jobs running
                    let (close, mut close_listener) = tokio::sync::mpsc::channel(32);
                    {
                        let mut running_jobs = running_jobs_arc.lock().unwrap();
                        running_jobs.insert(key.clone(), close);
                    }

                    tokio::task::spawn({
                        let key = key.clone();
                        let task_job = command_to_exec.take_task().unwrap();
                        let sender_commands = self.sender_commands.clone();
                        let running_jobs_arc = self.running_jobs.clone();
                        async move {
                            tokio::select! {
                                val = close_listener.recv() => {
                                    log::info!("    x ASYNC Job CLOSED");
                                }
                                val = task_job => {
                                    log::info!("    [ok] ASYNC Job Runned");
                                    if let Ok(command_final) = val {
                                        log::info!("    [ok2] Command of ASYNC Job Send");
                                        sender_commands.send(command_final).unwrap();
                                    }
                                }
                            }

                            // Removes from jobs running
                            let mut running_jobs = running_jobs_arc.lock().unwrap();
                            running_jobs.remove(&key);
                        }
                    });
                }
            }
        }
        Ok(())
    }
}
