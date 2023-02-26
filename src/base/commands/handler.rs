use std::{sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
}, time::Duration};

use crate::app::App;

use super::{Command, CommandTrait};

pub struct CommandHandler {
    sender_commands: Sender<Command>,
    listener_commands: Receiver<Command>,
}

impl CommandHandler {
    pub fn execute(app: &mut App, command: Command) -> Result<(), String> {
        command.execute(app)
    }

    pub fn init() -> Self {
        let (tx, rx): (
            Sender<Arc<Box<(dyn CommandTrait + Send + Sync)>>>,
            Receiver<Arc<Box<(dyn CommandTrait + Send + Sync)>>>,
        ) = mpsc::channel();

        Self {
            sender_commands: tx,
            listener_commands: rx,
        }
    }

    pub fn add(&mut self, command: Command) {
        log::info!("Added command");

        self.sender_commands.send(command).unwrap();
    }

    pub fn run(&mut self, app: &mut App) -> Result<(), String> {
        log::info!("Init runned");

        let mut i = 0;
        while let Ok(command_to_exec) = self.listener_commands.try_recv() {
            command_to_exec.execute(app).unwrap();

            log::info!("command {}", i);
            i += 1;

        }
        Ok(())
    }
}
