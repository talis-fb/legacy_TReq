use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
};

use crate::{app::App, base::commands::CommandType};

use super::{Command, CommandTrait};

pub struct CommandHandler {
    running_jobs: HashMap<String, tokio::sync::mpsc::Sender<()>>,

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
            running_jobs: HashMap::new(),
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
                    log::info!("Start to run a commad sync");
                    command_to_exec.execute(app).unwrap();
                    log::info!("End to run a commad sync");
                }
                CommandType::Async => {
                    log::info!("Start to run a commad ASYNC");
                    command_to_exec.execute(app).unwrap();

                    let key = command_to_exec.get_id();
                    let task_job = command_to_exec.take_task().unwrap();
                    let (close, mut close_listener) = tokio::sync::mpsc::channel(32);

                    self.running_jobs.insert(key, close);

                    let sender_commands = self.sender_commands.clone();
                    tokio::task::spawn(async move {
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
                    });
                }
            }
        }
        Ok(())
    }
}
