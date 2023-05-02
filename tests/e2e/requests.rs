use std::collections::HashMap;

use crate::mocks::mock_app::MockApp;
use crate::utils::set_external_editor_output;
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

#[tokio::test]
async fn should_create_only_one_tempfile_to_each_edit_body_or_header() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToRequestBody).await;

    assert_eq!(
        mock_app
            .runner
            .app
            .get_data_store()
            .config
            .files
            .lock()
            .unwrap()
            .get_map_files_temp_edition()
            .len(),
        0
    );

    set_external_editor_output(&mut mock_app, String::from(r#"{ "my_body": "something" }"#)).await;
    mock_app.exec(Actions::Edit).await;

    let req = mock_app.runner.app.get_data_store().get_request();
    assert_eq!(req.body, r#"{ "my_body": "something" }"#);

    {
        let files = mock_app
            .runner
            .app
            .get_data_store()
            .config
            .files
            .lock()
            .unwrap();

        let tempfiles_map = files.get_map_files_temp_edition();
        assert_eq!(tempfiles_map.len(), 1);
    }

    set_external_editor_output(&mut mock_app, String::from(r#"{ "my_body": "something" }"#)).await;
    mock_app.exec(Actions::Edit).await;

    {
        let files = mock_app
            .runner
            .app
            .get_data_store()
            .config
            .files
            .lock()
            .unwrap();

        let tempfiles_map = files.get_map_files_temp_edition();
        assert_eq!(tempfiles_map.len(), 1);
    }
}
