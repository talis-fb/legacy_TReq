use crate::mocks::mock_app::MockApp;
use treq::app::InputMode;
use treq::base::actions::Actions;

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
    mock_app.exec(Actions::TypingChar('<'));
    mock_app.exec(Actions::TypingChar('<'));
    mock_app.exec(Actions::TypingChar('<'));
    mock_app.exec(Actions::TypingClose);

    let request_name = mock_app.app.get_data_store().get_request().name.clone();
    assert_eq!(&request_name, "New Request<<<");

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

    for _ in 0..7 {
        mock_app.exec(Actions::TypingErase);
    }
    mock_app.exec(Actions::TypingChar('e'));
    mock_app.exec(Actions::TypingChar('d'));
    mock_app.exec(Actions::TypingChar('i'));
    mock_app.exec(Actions::TypingChar('t'));
    mock_app.exec(Actions::TypingChar('i'));
    mock_app.exec(Actions::TypingChar('n'));
    mock_app.exec(Actions::TypingChar('g'));
    mock_app.exec(Actions::TypingClose);

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
    mock_app.exec(Actions::TypingChar('<'));
    mock_app.exec(Actions::TypingChar('<'));
    mock_app.exec(Actions::TypingChar('<'));
    mock_app.exec(Actions::TypingClose);

    assert_eq!(
        get_request_names(&mock_app),
        vec!["New editing", "New Request", "New Request<<<",]
    );
}
