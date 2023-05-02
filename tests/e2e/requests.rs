use std::collections::HashMap;

use crate::mocks::file_factory::MockFileFactory;
use crate::mocks::mock_app::MockApp;
use crate::utils::{set_external_editor_output, set_input_mode_value};
use treq::app::InputMode;
use treq::base::actions::Actions;
use treq::base::web::request::Request;
use treq::input::input_handler::MockInputHandler;

#[tokio::test]
async fn should_edit_body_and_header_request() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToRequestBody).await;

    let req_default = Request::default();

    assert_eq!(
        req_default.body,
        mock_app.runner.app.get_data_store().get_request().body
    );
    assert_eq!(
        req_default.headers,
        mock_app.runner.app.get_data_store().get_request().headers
    );

    let mut input_handler = MockInputHandler::default();
    input_handler.expect_update().returning(|_| {});
    input_handler.expect_close_async_listener().returning(|| {});
    mock_app.runner.input_handler = input_handler;

    let req = mock_app.runner.app.get_data_store().get_request();
    assert_eq!(req.body, req_default.body);

    set_external_editor_output(&mut mock_app, String::from(r#"{ "my_body": "somethin" }"#)).await;
    mock_app.exec(Actions::Edit).await;

    let req = mock_app.runner.app.get_data_store().get_request();
    assert_eq!(req.body, r#"{ "my_body": "somethin" }"#);
    assert_eq!(req.headers, req_default.headers);

    mock_app.exec(Actions::Switch).await;

    set_external_editor_output(
        &mut mock_app,
        String::from(r#"{ "some_other_header": "something" }"#),
    )
    .await;
    mock_app.exec(Actions::Edit).await;

    let req = mock_app.runner.app.get_data_store().get_request();
    assert_eq!(req.body, r#"{ "my_body": "somethin" }"#);
    assert_eq!(
        req.headers,
        HashMap::from([("some_other_header".to_string(), "something".to_string())])
    );

    set_external_editor_output(
        &mut mock_app,
        String::from(r#"{ some_invalid_json: _something_ }"#),
    )
    .await;
    mock_app.exec(Actions::Edit).await;

    // Doens't change
    let req = mock_app.runner.app.get_data_store().get_request();
    assert_eq!(req.body, r#"{ "my_body": "somethin" }"#);
    assert_eq!(
        req.headers,
        HashMap::from([("some_other_header".to_string(), "something".to_string())])
    );
}
