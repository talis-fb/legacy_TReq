use crate::base::commands::{Command, Commands};

pub trait InputBuffer {
    fn block_reading(&mut self) -> Result<String, ()>;
}

pub struct InputKeyboardBuffer {
    pub value_backup: Option<String>,
    pub value: String,
    pub command: Command,
}
impl InputKeyboardBuffer {
    pub fn init() -> Self {
        InputKeyboardBuffer {
            value: String::new(),
            command: Commands::do_nothing(),
            value_backup: None,
        }
    }
    pub fn set_backup(&mut self, s: String) {
        self.value_backup = Some(s)
    }
    pub fn reset_to_backup(&mut self) {
        self.value = self.value_backup.clone().unwrap_or_default()
    }
}
