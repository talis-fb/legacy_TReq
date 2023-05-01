use crate::mocks::mock_app::MockApp;
use crate::utils::set_input_mode_value;
use treq::app::InputMode;
use treq::base::actions::Actions;

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
