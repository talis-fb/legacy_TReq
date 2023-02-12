use crate::commands::{Command, Commands};
use crate::App;

impl Commands {
    pub fn grow_right_ui() -> Command {
        |app: &mut App| {
            app.get_data_store_mut()
                .config
                .view
                .lock()
                .unwrap()
                .grow_right_block();
            Ok(())
        }
    }

    pub fn grow_left_ui() -> Command {
        |app: &mut App| {
            app.get_data_store_mut()
                .config
                .view
                .lock()
                .unwrap()
                .grow_left_block();
            Ok(())
        }
    }
}
