use std::collections::HashMap;
use std::time::Duration;

use crate::mocks::mock_app::MockApp;
use treq::app::InputMode;
use treq::base::actions::Actions;
use treq::base::web::client::WebClient;
use treq::base::web::repository::MockHttpClientRepository;
use treq::base::web::response::Response;

fn set_input_mode_value(mock_app: &mut MockApp, value: &str) {
    (0..100).for_each(|_| {
        mock_app.exec(Actions::TypingErase);
    });

    value.chars().for_each(|c| {
        mock_app.exec(Actions::TypingChar(c));
    });

    mock_app.exec(Actions::TypingClose)
}

#[test]
fn should_create_and_delete_tabs() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToTabList);

    let requests = mock_app.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 1);

    mock_app.exec(Actions::New);

    let requests = mock_app.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 2);

    mock_app.exec(Actions::New);
    mock_app.exec(Actions::New);

    let requests = mock_app.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 4);

    mock_app.exec(Actions::Delete);

    let requests = mock_app.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 3);
}

#[test]
fn should_delete_last_tab_creating_a_default() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToTabList);

    let requests = mock_app.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 1);

    mock_app.exec(Actions::Edit);

    set_input_mode_value(&mut mock_app, "Some new value");

    let request_name = mock_app.app.get_data_store().get_request().name.clone();
    assert_eq!(&request_name, "Some new value");

    mock_app.exec(Actions::Delete);

    // It stay with default Request
    let requests = mock_app.app.get_data_store().get_requests();
    assert_eq!(requests.len(), 1);

    let request_name = mock_app.app.get_data_store().get_request().name.clone();
    assert_eq!(&request_name, "New Request");
}

#[test]
fn should_create_and_move_between_tabs() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToTabList);

    let requests = mock_app.app.get_data_store().get_requests();
    let index_req = mock_app.app.get_data_store().request_ind();
    assert_eq!(requests.len(), 1);
    assert_eq!(index_req, 0);

    mock_app.exec(Actions::New);
    mock_app.exec(Actions::New);
    mock_app.exec(Actions::New);
    mock_app.exec(Actions::New);

    let requests = mock_app.app.get_data_store().get_requests();
    let index_req = mock_app.app.get_data_store().request_ind();
    assert_eq!(requests.len(), 5);
    assert_eq!(index_req, 4);

    mock_app.exec(Actions::GoToPreviousTab);
    let index_req = mock_app.app.get_data_store().request_ind();
    assert_eq!(index_req, 3);

    mock_app.exec(Actions::GoToPreviousTab);
    mock_app.exec(Actions::GoToPreviousTab);
    mock_app.exec(Actions::GoToPreviousTab);

    let index_req = mock_app.app.get_data_store().request_ind();
    assert_eq!(index_req, 0);

    // Goto Last
    mock_app.exec(Actions::GoToPreviousTab);
    let index_req = mock_app.app.get_data_store().request_ind();
    assert_eq!(index_req, 4);

    // Goto First
    mock_app.exec(Actions::GoToNextTab);
    let index_req = mock_app.app.get_data_store().request_ind();
    assert_eq!(index_req, 0);
}

#[test]
fn should_create_and_edit_tabs() {
    let mut mock_app = MockApp::init();

    mock_app.exec(Actions::GoToTabList);

    let requests = mock_app.app.get_data_store().get_requests();
    let index_req = mock_app.app.get_data_store().request_ind();
    assert_eq!(requests.len(), 1);
    assert_eq!(index_req, 0);

    assert_eq!(mock_app.app.get_mode(), InputMode::Normal);
    mock_app.exec(Actions::Edit);
    assert_eq!(mock_app.app.get_mode(), InputMode::Insert);

    assert_eq!(
        &mock_app.app.get_data_store().get_request().name,
        "New Request"
    );

    set_input_mode_value(&mut mock_app, "New editing");

    assert_eq!(mock_app.app.get_mode(), InputMode::Normal);

    fn get_request_names(mock_app: &MockApp) -> Vec<&String> {
        mock_app
            .app
            .get_data_store()
            .get_requests()
            .iter()
            .map(|it| &it.name)
            .collect()
    }

    assert_eq!(get_request_names(&mock_app), vec!["New editing"]);

    mock_app.exec(Actions::New);
    mock_app.exec(Actions::New);

    assert_eq!(
        get_request_names(&mock_app),
        vec!["New editing", "New Request", "New Request",]
    );

    mock_app.exec(Actions::Edit);

    set_input_mode_value(&mut mock_app, "Edited name");

    assert_eq!(
        get_request_names(&mock_app),
        vec!["New editing", "New Request", "Edited name",]
    );
}

#[test]
fn should_manage_enviroment() {
    let mut mock_app = MockApp::init();
    mock_app.exec(Actions::GoToEnvironment);

    // Goto Global Environments
    mock_app.exec(Actions::Switch);

    assert_eq!(
        mock_app.app.get_data_store().environment.global,
        HashMap::from([])
    );

    mock_app.exec(Actions::Delete);

    assert_eq!(
        mock_app.app.get_data_store().environment.global,
        HashMap::from([])
    );

    (0..100).for_each(|_| {
        mock_app.exec(Actions::Delete);
    });

    assert_eq!(
        mock_app.app.get_data_store().environment.global,
        HashMap::from([])
    );
}

#[tokio::test]
async fn should_replace_var_fields_url() {
    let mut mock_app = MockApp::init();
    mock_app.exec(Actions::GoToEnvironment);

    // Goto Global Environments
    mock_app.exec(Actions::Switch);

    // Create and set key variable
    mock_app.exec(Actions::New);
    set_input_mode_value(&mut mock_app, "variable_name1");

    // Set value variable
    mock_app.exec(Actions::Edit);
    set_input_mode_value(&mut mock_app, "VALUE_INSIDE_VARIABLE");

    mock_app.exec(Actions::Quit);

    mock_app.exec(Actions::GoToUrl);
    mock_app.exec(Actions::Edit);
    set_input_mode_value(&mut mock_app, "google.com/search/{{ variable_name1 }}");

    assert_eq!(
        &mock_app.app.get_data_store().get_request().url,
        "google.com/search/{{ variable_name1 }}"
    );

    let mut http_client = MockHttpClientRepository::new();
    http_client.expect_call_get().returning(|url, _| {
        let mut res = Response::default();
        res.body = format!("URL: {}", url);
        Ok(res)
    });

    mock_app.app.set_web_client(WebClient::init(http_client));

    // Submit and await async functions be runned
    mock_app.exec(Actions::Submit);
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(
        &mock_app
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

    println!("{:?}", mock_app.app.get_data_store().environment.global);
    mock_app.exec(Actions::Quit);

    mock_app.exec(Actions::GoToRequestBody);

    mock_app.buffer_input = r#"
            {
                "my_name": "{{ name }}",
                "mac": "{{ mac | upper }}",
                "mac_formatted": "{{ mac | lower | replace(from=":", to="") }}",
            }
        "#
    .to_string();

    mock_app.exec(Actions::Edit);

    // Change to POST Method
    mock_app.exec(Actions::GoToUrl);
    mock_app.exec(Actions::Switch);

    println!("####");
    println!("{:?}", mock_app.app.get_data_store().get_request());
    println!("####");

    let mut http_client = MockHttpClientRepository::new();
    http_client.expect_call_post().returning(|_, _, body| {
        let mut res = Response::default();
        res.body = body;
        Ok(res)
    });

    mock_app.app.set_web_client(WebClient::init(http_client));

    // Submit and await async functions be runned
    mock_app.exec(Actions::Submit);
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert_eq!(
        &mock_app
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
