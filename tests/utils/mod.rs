use crate::mocks::mock_app::MockApp;
use treq::base::actions::Actions;

pub async fn set_input_mode_value(mock_app: &mut MockApp, value: &str) {
    mock_app.exec(Actions::TypingClearAll).await;

    for c in value.chars() {
        mock_app.exec(Actions::TypingChar(c)).await;
    }

    mock_app.exec(Actions::TypingClose).await;
}
