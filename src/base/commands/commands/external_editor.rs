use crate::app::App;
use crate::base::commands::CommandTrait;
use crate::base::commands::{Command, Commands};
use crate::base::os::os_commands::OsCommand;
use crate::utils::custom_types::uuid::UUID;
use std::sync::Arc;

impl Commands {
    pub fn open_editor_to_buffer(
        command: Command,
        initial_value: String,
        uuid: Option<UUID>,
    ) -> Command {
        struct S {
            command: Command,
            uuid: Option<UUID>,
            initial_value: String,
        }
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let path;
                {
                    let store = app.get_data_store_mut();
                    let mut file_handler = store.config.files.lock().unwrap();
                    store.input_buffer.set_backup(self.initial_value.clone());

                    path = if let Some(uuid) = &self.uuid {
                        file_handler
                            .get_map_files_temp_edition()
                            .get(uuid)
                            .unwrap()
                            .get_path()
                    } else {
                        let temp_file_to_edit = file_handler
                            .file_factory
                            .as_mut()
                            .unwrap()
                            .create_temp_file(UUID::new(), self.initial_value.clone())?;
                        let path = temp_file_to_edit.get_path();

                        file_handler.add_temp_edition(temp_file_to_edit);
                        path
                    };
                }

                let command_pos_edit = app
                    .os_commands_factory
                    .as_ref()
                    .unwrap()
                    .external_editor(path, self.command.clone())
                    .unwrap();

                let os_command = OsCommand::Sync(Arc::new(command_pos_edit));

                tokio::task::spawn({
                    let sender = app.os_commands_queue.as_ref().unwrap().clone();
                    async move {
                        sender.send(os_command).await.ok();
                    }
                });

                Ok(())
            }
        }

        Commands::from(S {
            command,
            uuid,
            initial_value,
        })
    }
}
