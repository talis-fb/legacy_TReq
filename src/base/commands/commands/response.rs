use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn edit_response_vim() -> Command {
        |app: &mut App| {
            let response = app.get_data_store().get_response().lock().unwrap().clone();

            // It opens Editor with response Data, and does nothing when finished
            app.set_vim_mode_with_command(|app: &mut App| Ok(()), response.body);

            Ok(())
        }
    }
}
