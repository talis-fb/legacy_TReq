use tokio::sync::mpsc::Receiver;

use crate::{
    app::{App, InputMode},
    base::{
        actions::Actions,
        commands::{handler::CommandHandler, Command, Commands},
        os::os_commands::OsCommand,
    },
    input::input_handler::{InputDefaultHandler, InputHandler},
    view::{ui::UI, UiTrait},
};

pub struct Runner<View: UiTrait, Input: InputHandler> {
    pub app: App,
    pub command_handler: CommandHandler,
    pub input_handler: Input,
    pub view: View,

    // Receivers
    pub action_queue: Option<Receiver<Actions>>,
    pub commands_queue: Option<Receiver<Command>>,
    pub os_commands_queue: Option<Receiver<OsCommand>>,
}

impl<View, Input> Runner<View, Input>
where
    View: UiTrait,
    Input: InputHandler,
{
    pub fn init(
        app: App,
        command_handler: CommandHandler,
        input_handler: Input,
        view: View,
    ) -> Self {
        Self {
            app,
            command_handler,
            input_handler,
            view,
            action_queue: None,
            commands_queue: None,
            os_commands_queue: None,
        }
    }

    pub fn set_receiver_actions_queue(&mut self, receiver: Receiver<Actions>) {
        self.action_queue = Some(receiver);
    }
    pub fn set_receiver_commands_queue(&mut self, receiver: Receiver<Command>) {
        self.commands_queue = Some(receiver);
    }
    pub fn set_receiver_os_commands_queue(&mut self, receiver: Receiver<OsCommand>) {
        self.os_commands_queue = Some(receiver);
    }
}

impl<View, Input> Runner<View, Input>
where
    View: UiTrait,
    Input: InputHandler,
{
    pub fn is_finished(&self) -> bool {
        self.app.is_finished
    }

    pub fn render(&mut self) {
        let store = self.app.get_data_store();
        self.view.render(store);
    }

    pub fn close(&mut self) {
        self.input_handler.close_async_listener();
        self.view.close();
    }

    pub fn update_input_handler(&mut self) {
        let input_mode = self.app.get_mode();
        self.input_handler.update(input_mode);
    }

    pub async fn proccess(&mut self) {
        let actions = self.action_queue.as_mut().unwrap();
        let commands = self.commands_queue.as_mut().unwrap();
        let os_commands = self.os_commands_queue.as_mut().unwrap();

        log::info!("############## AGUARDE ########");

        tokio::select! {
            action = actions.recv() => {
                log::info!("Action {:?}", action);

                let command = self.app
                    .get_command_of_action(action.unwrap())
                    .unwrap_or(Commands::do_nothing());


                // Add Command to queue
                self.command_handler.add(command);
            }
            command = commands.recv() => {
                log::info!("TOME COMANDO NNESSA PORRA");
                let command_result = self.command_handler.run(command.unwrap(), &mut self.app);

                if let Err(e) = command_result {
                    self.app.get_data_store_mut()
                        .set_log_error(String::from("COMMAND ERROR"), e.to_string())
                }
            }
            os_command = os_commands.recv() => {
                match os_command.unwrap() {
                    OsCommand::Sync(comm) => {
                        self.view.close();
                        self.input_handler.close_async_listener();

                        let commands_sender = self.command_handler.get_sender();

                        log::info!("BEFORE command vim");
                        let output = comm.exec(commands_sender);
                        log::info!("AFTER command vim");

                        if let Err(e) = output {
                            self.app.get_data_store_mut()
                                .set_log_error(String::from("OS COMMAND ERROR"), e.to_string())
                        }

                        // self.input_handler.close_async_listener();
                        self.app.set_mode(InputMode::Normal);
                        self.input_handler.update(InputMode::Normal);

                        self.view.restart();
                        self.view.render(self.app.get_data_store());

                        while actions.try_recv().is_ok() {
                            log::info!("Clear queue");
                        }

                        log::info!("@@ Terminou OS @@");

                    }

                    OsCommand::Async(comm) => {
                        let commands_sender = self.command_handler.get_sender();
                        tokio::task::spawn(async move {
                            comm.exec(commands_sender).unwrap();
                        });
                    }
                }
            }

        }
    }
}
