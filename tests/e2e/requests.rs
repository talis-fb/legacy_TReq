use crate::mocks::file_factory::MockFileFactory;
use crate::mocks::mock_app::MockApp;
use crate::utils::set_input_mode_value;
use treq::app::InputMode;
use treq::base::actions::Actions;
use treq::base::os::os_commands::factory::MockOsCommandFactory;
use treq::base::os::os_commands::MockOsCommandTrait;
use treq::base::web::request::Request;
use treq::input::input_handler::MockInputHandler;

#[tokio::test]
async fn should_edit_body_request() {
    let mut mock_app = MockApp::init();
}

#[tokio::test]
async fn should_edit_header_request() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToRequestBody).await;

    let req_default = Request::default();

    assert_eq!(
        "{}",
        mock_app.runner.app.get_data_store().get_request().body
    );
    assert_eq!(
        req_default.headers,
        mock_app.runner.app.get_data_store().get_request().headers
    );

    // let mut file_factory = MockFileFactory::default();

    let mut input_handler = MockInputHandler::default();
    input_handler.expect_update().returning(|_| {});
    input_handler.expect_close_async_listener().returning(|| {});

    // let mut path_called_to_body = Rc

    let mut os_command_factory = MockOsCommandFactory::default();
    os_command_factory
        .expect_external_editor()
        .return_once(move |_, _| {
            // path_called_to_body.push(path);
            Ok(Box::new(MockOsCommandTrait::default()))
        });

    // Set mocks on app
    mock_app
        .runner
        .app
        .set_os_commands_factory(os_command_factory);
    mock_app.runner.input_handler = input_handler;

    // mock_app.exec(Actions::Edit).await;
}
