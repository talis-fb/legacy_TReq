use crate::mocks::mock_app::MockApp;
use crate::utils::{set_external_editor_output, set_input_mode_value};
use std::collections::HashMap;
use std::time::Duration;
use treq::base::actions::Actions;
use treq::base::web::client::WebClient;
use treq::base::web::repository::MockHttpClientRepository;
use treq::base::web::response::Response;

#[tokio::test]
async fn should_manage_enviroment() {
    let mut mock_app = MockApp::init();
    mock_app.exec(Actions::GoToEnvironment).await;

    // Goto Global Environments
    mock_app.exec(Actions::Switch).await;

    assert_eq!(
        mock_app.runner.app.get_data_store().environment.global,
        HashMap::from([])
    );

    mock_app.exec(Actions::Delete).await;

    assert_eq!(
        mock_app.runner.app.get_data_store().environment.global,
        HashMap::from([])
    );

    mock_app.exec(Actions::Delete).await;
    mock_app.exec(Actions::Delete).await;
    mock_app.exec(Actions::Delete).await;
    mock_app.exec(Actions::Delete).await;
    mock_app.exec(Actions::Delete).await;
    mock_app.exec(Actions::Delete).await;
    mock_app.exec(Actions::Delete).await;

    assert_eq!(
        mock_app.runner.app.get_data_store().environment.global,
        HashMap::from([])
    );
}

#[tokio::test]
async fn should_replace_var_fields_url() {
    let mut mock_app = MockApp::init();
    mock_app.exec(Actions::GoToEnvironment).await;

    // Goto Global Environments
    mock_app.exec(Actions::Switch).await;

    // Create and set key variable
    mock_app.exec(Actions::New).await;
    set_input_mode_value(&mut mock_app, "variable_name1").await;

    // Set value variable
    mock_app.exec(Actions::Edit).await;
    set_input_mode_value(&mut mock_app, "VALUE_INSIDE_VARIABLE").await;

    mock_app.exec(Actions::Quit).await;

    mock_app.exec(Actions::GoToUrl).await;
    mock_app.exec(Actions::Edit).await;
    set_input_mode_value(&mut mock_app, "google.com/search/{{ variable_name1 }}").await;

    assert_eq!(
        &mock_app.runner.app.get_data_store().get_request().url,
        "google.com/search/{{ variable_name1 }}"
    );

    let mut http_client = MockHttpClientRepository::new();
    http_client.expect_call_get().returning(|url, _| {
        let mut res = Response::default();
        res.body = format!("URL: {}", url);
        Ok(res)
    });

    mock_app
        .runner
        .app
        .set_web_client(WebClient::init(http_client));

    // Submit and await async functions be runned
    mock_app.exec(Actions::Submit).await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(
        &mock_app
            .runner
            .app
            .get_data_store()
            .get_response()
            .lock()
            .unwrap()
            .body,
        "URL: http://google.com/search/VALUE_INSIDE_VARIABLE"
    );
}

#[tokio::test]
async fn should_replace_var_fields_body() {
    let mut mock_app = MockApp::init();
    mock_app.exec(Actions::GoToEnvironment).await;

    // Goto Global Environments
    mock_app.exec(Actions::Switch).await;

    // Create and set key variable
    mock_app.exec(Actions::New).await;
    set_input_mode_value(&mut mock_app, "variable_name").await;

    // Set value variable
    mock_app.exec(Actions::Edit).await;
    set_input_mode_value(&mut mock_app, "value_inside_variable").await;

    mock_app.exec(Actions::Quit).await;

    // Change to POST
    mock_app.exec(Actions::GoToUrl).await;
    mock_app.exec(Actions::Switch).await;

    mock_app.exec(Actions::GoToRequestBody).await;
    set_external_editor_output(
        &mut mock_app,
        "Some body with {{ variable_name }} value".to_string(),
    )
    .await;
    mock_app.exec(Actions::Edit).await;

    assert_eq!(
        &mock_app.runner.app.get_data_store().get_request().body,
        "Some body with {{ variable_name }} value"
    );

    let mut http_client = MockHttpClientRepository::new();
    http_client
        .expect_call_post()
        .returning(|_url, headers, body| {
            let mut res = Response::default();
            res.headers = headers;
            res.body = format!("Body: {}", body);
            Ok(res)
        });

    mock_app
        .runner
        .app
        .set_web_client(WebClient::init(http_client));

    // Submit and await async functions be runned
    mock_app.exec(Actions::Submit).await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    let response_body = {
        let res_mutex = mock_app.runner.app.get_data_store().get_response();
        let res = res_mutex.lock().unwrap();
        res.body.clone()
    };

    assert_eq!(
        response_body,
        "Body: Some body with value_inside_variable value"
    );

    // Change HEADER
    mock_app.exec(Actions::GoToRequestBody).await;
    mock_app.exec(Actions::Switch).await;
    set_external_editor_output(
        &mut mock_app,
        r#"{ "some_header": "{{ variable_name | upper }}" }"#.to_string(),
    )
    .await;
    mock_app.exec(Actions::Edit).await;

    // SUBMIT
    mock_app.exec(Actions::Submit).await;
    tokio::time::sleep(Duration::from_millis(50)).await;
    let (response_body, response_header) = {
        let res_mutex = mock_app.runner.app.get_data_store().get_response();
        let res = res_mutex.lock().unwrap();
        (res.body.clone(), res.headers.clone())
    };

    assert_eq!(
        &response_body,
        "Body: Some body with value_inside_variable value"
    );
    assert_eq!(
        response_header,
        HashMap::from([(
            "some_header".to_string(),
            "VALUE_INSIDE_VARIABLE".to_string()
        )])
    );
}
