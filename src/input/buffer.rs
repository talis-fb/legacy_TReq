use crate::base::commands::{Command, Commands};

pub struct InputKeyboardBuffer {
    pub value_backup: Option<String>,
    pub value: String,
    pub command: Command,

    cursor: usize,
}
impl InputKeyboardBuffer {
    pub fn init() -> Self {
        InputKeyboardBuffer {
            value: String::new(),
            command: Commands::do_nothing(),
            value_backup: None,
            cursor: 0,
        }
    }
    pub fn set_backup(&mut self, s: String) {
        self.value_backup = Some(s)
    }
    pub fn reset_to_backup(&mut self) {
        self.value = self.value_backup.clone().unwrap_or_default()
    }
}

// Managing editing inside app
impl InputKeyboardBuffer {
    pub fn set_cursor(&mut self, i: usize) {
        let pos = i.min(self.value.chars().count());
        if pos != self.cursor {
            self.cursor = pos;
        }
    }
    pub fn insert_char(&mut self, ch: char) {
        if self.cursor == self.value.chars().count() {
            self.value.push(ch);
        } else {
            self.value = self
                .value
                .chars()
                .take(self.cursor)
                .chain(std::iter::once(ch).chain(self.value.chars().skip(self.cursor)))
                .collect();
        }
        self.cursor += 1;
    }

    pub fn delete_prev_char(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            self.value = self
                .value
                .chars()
                .enumerate()
                .filter(|(i, _)| i != &self.cursor)
                .map(|(_, c)| c)
                .collect();
        }
    }
    pub fn delete_next_char(&mut self) {
        if self.cursor != self.value.chars().count() {
            self.value = self
                .value
                .chars()
                .enumerate()
                .filter(|(i, _)| i != &self.cursor)
                .map(|(_, c)| c)
                .collect();
        }
    }

    pub fn delete_till_end(&mut self) {
        self.value = self.value.chars().take(self.cursor).collect();
    }

    pub fn go_to_prev_char(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }
    pub fn go_to_next_char(&mut self) {
        if self.cursor == self.value.chars().count() {
            self.cursor += 1;
        }
    }
    pub fn go_to_start(&mut self) {
        if self.cursor > 0 {
            self.cursor = 0;
        }
    }
    pub fn go_to_end(&mut self) {
        let count = self.value.chars().count();
        if self.cursor != count {
            self.cursor = count;
        }
    }

    pub fn go_to_prev_word(&mut self) {
        if self.cursor > 0 {
            self.cursor = self
                .value
                .chars()
                .rev()
                .skip(self.value.chars().count().max(self.cursor) - self.cursor)
                .skip_while(|c| !c.is_alphanumeric())
                .skip_while(|c| c.is_alphanumeric())
                .count();
        }
    }
    pub fn go_to_next_word(&mut self) {
        if self.cursor != self.value.chars().count() {
            self.cursor = self
                .value
                .chars()
                .enumerate()
                .skip(self.cursor)
                .skip_while(|(_, c)| c.is_alphanumeric())
                .find(|(_, c)| c.is_alphanumeric())
                .map(|(i, _)| i)
                .unwrap_or_else(|| self.value.chars().count());
        }
    }

    // pub fn DeletePrevWord() {
    // }
    // pub fn DeleteNextWord() {
    // }
    // pub fn DeleteLine() {
    // }
}
