use crate::mocks::mock_app::MockApp;
use treq::base::{actions::Actions, os::os_commands::{MockOsCommandTrait, factory::MockOsCommandFactory}, commands::Commands};

pub async fn set_input_mode_value(mock_app: &mut MockApp, value: &str) {
    mock_app.exec(Actions::TypingClearAll).await;

    for c in value.chars() {
        mock_app.exec(Actions::TypingChar(c)).await;
    }

    mock_app.exec(Actions::TypingClose).await;
}

pub async fn set_external_editor_output(mock_app: &mut MockApp, value: String) {
    let mut os_command_factory = MockOsCommandFactory::default();
    os_command_factory
        .expect_external_editor()
        .return_once(move |_, command| {
            let mut editor_to_edit_body = MockOsCommandTrait::default();
            editor_to_edit_body
                .expect_exec()
                .times(1)
                .returning(move |sender| {
                    let set_input_buffer = Commands::set_input_buffer(value.clone());
                    let exec_input_buffer = command.clone();

                    tokio::task::spawn(async move {
                        sender.send(set_input_buffer).await.ok();
                        sender.send(exec_input_buffer).await.ok();
                    });

                    Ok(())
                });

            Ok(Box::new(editor_to_edit_body))
        });

    // Set mocks on app
    mock_app
        .runner
        .app
        .set_os_commands_factory(os_command_factory);
}
