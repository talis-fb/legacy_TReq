use crate::app::App;
use crate::base::commands::CommandTrait;
use crate::base::commands::{Command, Commands};
use crate::base::os::os_commands::OsCommand;
use crate::utils::custom_types::uuid::UUID;
use std::sync::Arc;

impl Commands {
    pub fn open_editor_to_buffer(
        command: Command,
        uuid: Option<UUID>,
        initial_value: Option<String>,
    ) -> Command {
        struct S {
            command: Command,
            uuid: Option<UUID>,
            initial_value: Option<String>,
        }
        impl CommandTrait for S {
            fn execute(&self, app: &mut App) -> Result<(), String> {
                let path_file_to_edit;
                {
                    let store = app.get_data_store_mut();
                    let mut file_handler = store.config.files.lock().unwrap();

                    let uuid = self.uuid.clone().unwrap_or_default();

                    if file_handler
                        .get_map_files_temp_edition()
                        .get(&uuid)
                        .is_none()
                    {
                        let temp_file_to_edit = file_handler
                            .file_factory
                            .as_mut()
                            .unwrap()
                            .create_temp_file(
                                uuid.clone(),
                                self.initial_value.clone().unwrap_or_default(),
                            )
                            .unwrap();

                        file_handler.add_temp_edition_with_id(temp_file_to_edit, uuid.clone());

                        file_handler
                            .get_map_files_temp_edition()
                            .get(&uuid)
                            .unwrap();
                    }

                    if let Some(value) = &self.initial_value {
                        store.input_buffer.set_backup(value.clone());
                        file_handler.save_content_temp_file(&uuid, value.clone())?;
                    }

                    path_file_to_edit = file_handler
                        .get_map_files_temp_edition()
                        .get(&uuid)
                        .ok_or("Error getting file inside FileHandler")?
                        .get_path();
                }

                let os_factory = app.os_commands_factory.as_ref().unwrap();
                let os_queue = app.os_commands_queue.as_ref().unwrap();

                let command_pos_edit = os_factory
                    .external_editor(path_file_to_edit, self.command.clone())
                    .unwrap();

                let os_command = OsCommand::Sync(Arc::new(command_pos_edit));

                tokio::task::spawn({
                    let sender = os_queue.clone();
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
