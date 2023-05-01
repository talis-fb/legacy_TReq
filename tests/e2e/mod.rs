pub mod tabs;
pub mod variables;
pub mod requests;

// async fn set_input_mode_value(mock_app: &mut MockApp, value: &str) {
//     for _ in 0..50 {
//         mock_app.exec(Actions::TypingErase).await;
//     }
//     
//     for c in value.chars() {
//         mock_app.exec(Actions::TypingChar(c)).await;
//     }
//
//     mock_app.exec(Actions::TypingClose).await;
// }






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
