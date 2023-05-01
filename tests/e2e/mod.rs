use std::collections::HashMap;
use std::time::Duration;

use crate::mocks::mock_app::MockApp;
use treq::app::InputMode;
use treq::base::actions::Actions;
use treq::base::os::os_commands::MockOsCommandTrait;
use treq::base::web::client::WebClient;
use treq::base::web::repository::MockHttpClientRepository;
use treq::base::web::response::Response;

async fn set_input_mode_value(mock_app: &mut MockApp, value: &str) {
    for _ in 0..50 {
        mock_app.exec(Actions::TypingErase).await;
    }
    
    for c in value.chars() {
        mock_app.exec(Actions::TypingChar(c)).await;
    }

    mock_app.exec(Actions::TypingClose).await;
}

#[tokio::test]
async fn should_create_and_delete_tabs() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToTabList).await;

    let requests = mock_app.runner.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 1);

    mock_app.exec(Actions::New).await;

    let requests = mock_app.runner.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 2);

    mock_app.exec(Actions::New).await;
    mock_app.exec(Actions::New).await;

    let requests = mock_app.runner.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 4);

    mock_app.exec(Actions::Delete).await;

    let requests = mock_app.runner.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 3);
}

#[tokio::test]
async fn should_delete_last_tab_creating_a_default() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToTabList).await;

    let requests = mock_app.runner.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 1);

    mock_app.exec(Actions::Edit).await;

    set_input_mode_value(&mut mock_app, "Some new value").await;

    let request_name = mock_app.runner.app.get_data_store().get_request().name.clone();
    assert_eq!(&request_name, "Some new value");

    mock_app.exec(Actions::Delete).await;

    // It stay with default Request
    let requests = mock_app.runner.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 1);

    let request_name = mock_app.runner.app.get_data_store().get_request().name.clone();
    assert_eq!(&request_name, "New Request");
}

#[tokio::test]
async fn should_create_and_move_between_tabs() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToTabList).await;

    let requests = mock_app.runner.app.get_data_store().get_requests();
    let index_req = mock_app.runner.app.get_data_store().request_ind();
    assert_eq!(requests.len(), 1);
    assert_eq!(index_req, 0);

    mock_app.exec(Actions::New).await;
    mock_app.exec(Actions::New).await;
    mock_app.exec(Actions::New).await;
    mock_app.exec(Actions::New).await;

    let requests = mock_app.runner.app.get_data_store().get_requests();
    let index_req = mock_app.runner.app.get_data_store().request_ind();
    assert_eq!(requests.len(), 5);
    assert_eq!(index_req, 4);

    mock_app.exec(Actions::GoToPreviousTab).await;
    let index_req = mock_app.runner.app.get_data_store().request_ind();
    assert_eq!(index_req, 3);

    mock_app.exec(Actions::GoToPreviousTab).await;
    mock_app.exec(Actions::GoToPreviousTab).await;
    mock_app.exec(Actions::GoToPreviousTab).await;

    let index_req = mock_app.runner.app.get_data_store().request_ind();
    assert_eq!(index_req, 0);

    // Goto Last
    mock_app.exec(Actions::GoToPreviousTab).await;
    let index_req = mock_app.runner.app.get_data_store().request_ind();
    assert_eq!(index_req, 4);

    // Goto First
    mock_app.exec(Actions::GoToNextTab).await;
    let index_req = mock_app.runner.app.get_data_store().request_ind();
    assert_eq!(index_req, 0);
}

#[tokio::test]
async fn should_create_and_edit_tabs() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToTabList).await;

    let requests = mock_app.runner.app.get_data_store().get_requests();
    let index_req = mock_app.runner.app.get_data_store().request_ind();
    assert_eq!(requests.len(), 1);
    assert_eq!(index_req, 0);

    assert_eq!(mock_app.runner.app.get_mode(), InputMode::Normal);
    mock_app.exec(Actions::Edit).await;
    assert_eq!(mock_app.runner.app.get_mode(), InputMode::Insert);

    assert_eq!(
        &mock_app.runner.app.get_data_store().get_request().name,
        "New Request"
    );

    set_input_mode_value(&mut mock_app, "New editing").await;

    assert_eq!(mock_app.runner.app.get_mode(), InputMode::Normal);

    fn get_request_names(mock_app: &MockApp) -> Vec<&String> {
        mock_app
            .runner.app
            .get_data_store()
            .get_requests()
            .iter()
            .map(|it| &it.name)
            .collect()
    }

    assert_eq!(get_request_names(&mock_app), vec!["New editing"]);

    mock_app.exec(Actions::New).await;
    mock_app.exec(Actions::New).await;

    assert_eq!(
        get_request_names(&mock_app),
        vec!["New editing", "New Request", "New Request",]
    );

    mock_app.exec(Actions::Edit).await;

    set_input_mode_value(&mut mock_app, "Edited name").await;

    assert_eq!(
        get_request_names(&mock_app),
        vec!["New editing", "New Request", "Edited name",]
    );
}

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

    // (0..100).for_each(|_| {
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

    mock_app.runner.app.set_web_client(WebClient::init(http_client));

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



/*

#[tokio::test]
async fn should_template_work_with_body() {
    let mut mock_app = MockApp::init();
    mock_app.exec(Actions::GoToEnvironment);

    // Goto Global Environments
    mock_app.exec(Actions::Switch);

    // Create and set key variable
    mock_app.exec(Actions::New);
    set_input_mode_value(&mut mock_app, "name");
    mock_app.exec(Actions::Edit);
    set_input_mode_value(&mut mock_app, "James Web");

    mock_app.exec(Actions::New);
    set_input_mode_value(&mut mock_app, "mac");

    mock_app.exec(Actions::Down);

    mock_app.exec(Actions::Edit);
    set_input_mode_value(&mut mock_app, "dd:01:eF:BB:AA:21");

    println!("{:?}", mock_app.runner.app.get_data_store().environment.global);
    mock_app.exec(Actions::Quit);

    mock_app.exec(Actions::GoToRequestBody);

    mock_app.set_output_sync_external_editor(
        r#"
            {
                "my_name": "{{ name }}",
                "mac": "{{ mac | upper }}",
                "mac_formatted": "{{ mac | lower | replace(from=":", to="") }}",
            }
        "#
        .to_string(),
    );

    // mock_app.buffer_input = r#"
    //         {
    //             "my_name": "{{ name }}",
    //             "mac": "{{ mac | upper }}",
    //             "mac_formatted": "{{ mac | lower | replace(from=":", to="") }}",
    //         }
    //     "#
    // .to_string();

    mock_app.exec(Actions::Edit);

    // Change to POST Method
    mock_app.exec(Actions::GoToUrl);
    mock_app.exec(Actions::Switch);

    println!("####");
    println!("{:?}", mock_app.runner.app.get_data_store().get_request());
    println!("####");

    let mut http_client = MockHttpClientRepository::new();
    http_client.expect_call_post().returning(|_, _, body| {
        let mut res = Response::default();
        res.body = body;
        Ok(res)
    });

    mock_app.runner.app.set_web_client(WebClient::init(http_client));

    // Submit and await async functions be runned
    mock_app.exec(Actions::Submit);
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
        r#"
            {
                "my_name": "James Web",
                "mac": "DD:01:EF:BB:AA:21",
                "mac_formatted": "dd01efbbaa21",
            }
        "#
    );
}

#[test]
fn should_edit_body_and_headers_request_corretly() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToRequestBody);

    // mock_app.buffer_input = r#" { "body": something is not a valid json } "#.to_string();
    mock_app.set_output_sync_external_editor(
        r#" { "body": something is not a valid json } "#.to_string(),
    );
    mock_app.exec(Actions::Edit);

    // Needs to re-render to update
    mock_app.exec(Actions::Null);

    assert_eq!(
        &mock_app.runner.app.get_data_store().get_request().body,
        r#" { "body": something is not a valid json } "#
    );

    mock_app.exec(Actions::Switch);

    // mock_app.buffer_input = r#" { "Auth": "none" } "#.to_string();
    // mock_app.set_output_sync_external_editor(r#" { "Auth": "none" } "#.to_string());

    let now_headers = mock_app.runner.app.get_data_store().get_request().headers.clone();

    // let mut editor = MockOsCommand::default();
    // editor
    //     .expect_sync_open()
    //     .withf(|path| {
    //     })
    //     // .with(eq(serde_json::to_string_pretty(&now_headers).unwrap()));

    // mock_app.set_external_editor(editor);

    mock_app.exec(Actions::Edit);

    // Needs to re-render to update
    mock_app.exec(Actions::Null);

    assert_eq!(
        &mock_app.runner.app.get_data_store().get_request().body,
        r#" { "body": something is not a valid json } "#
    );

    assert_eq!(
        mock_app.runner.app.get_data_store().get_request().headers,
        HashMap::from([("Auth".to_string(), "none".to_string())])
    );
}

*/
