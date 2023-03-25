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
    pub fn get_cursor(&self) -> usize {
        self.cursor
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

    pub fn set_value(&mut self, value: String) {
        self.value = value.clone();
        self.value_backup = Some(value.clone());
        self.cursor = self.value.chars().count();
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
        if self.cursor != self.value.chars().count() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_init_and_be_defined() {
        let buffer = InputKeyboardBuffer::init();

        assert_eq!(buffer.value, "".to_string());
        assert_eq!(buffer.get_cursor(), 0);
    }

    #[test]
    fn should_execute_corretly_set_cursor() {
        let mut buffer = InputKeyboardBuffer::init();

        buffer.set_value("Hello World".to_string());

        // Hello World<>
        assert_eq!(buffer.get_cursor(), 11);

        buffer.set_cursor(2);

        // He<l>lo World
        assert_eq!(buffer.value, "Hello World".to_string());
        assert_eq!(buffer.get_cursor(), 2);

        buffer.set_cursor(33);

        // Hello World<>
        assert_eq!(buffer.value, "Hello World".to_string());
        assert_eq!(buffer.get_cursor(), 11);

        buffer.set_cursor(6);

        // Hello <W>orld
        assert_eq!(buffer.value, "Hello World".to_string());
        assert_eq!(buffer.get_cursor(), 6);
    }

    #[test]
    fn should_change_cursor_with_set_value() {
        let mut buffer = InputKeyboardBuffer::init();
        buffer.set_value("Hello World".to_string());
        assert_eq!(buffer.get_cursor(), 11);

        buffer.set_value("Another Value with length 28".to_string());

        assert_eq!(buffer.get_cursor(), 28);
    }

    #[test]
    fn should_insert_corretly() {
        let mut buffer = InputKeyboardBuffer::init();
        buffer.set_value("Hello World".to_string());

        buffer.insert_char('X');

        assert_eq!(buffer.value, "Hello WorldX".to_string());
        assert_eq!(buffer.get_cursor(), 12);
    }

    #[test]
    fn should_delete_corretly() {
        let mut buffer = InputKeyboardBuffer::init();
        buffer.set_value("Hello World".to_string());

        buffer.delete_prev_char();

        assert_eq!(buffer.value, "Hello Worl".to_string());
        assert_eq!(buffer.get_cursor(), 10);


        // Hel<l>o Worl
        buffer.set_cursor(3);


        // He<l>o Worl
        buffer.delete_prev_char();
        assert_eq!(buffer.value, "Helo Worl".to_string());
        assert_eq!(buffer.get_cursor(), 2);


        // He<o> World
        buffer.delete_next_char();
        assert_eq!(buffer.value, "Heo Worl".to_string());
        assert_eq!(buffer.get_cursor(), 2);


        // He<>
        buffer.delete_till_end();
        assert_eq!(buffer.value, "He".to_string());
        assert_eq!(buffer.get_cursor(), 2);
    }

    #[test]
    fn should_jump_corretly() {
        let mut buffer = InputKeyboardBuffer::init();
        buffer.set_value("Hello World".to_string());

        fn is_value_same_initial(buffer: &InputKeyboardBuffer){
            assert_eq!(buffer.value, "Hello World".to_string());
        }

        // Hello World<>
        is_value_same_initial(&buffer);
        assert_eq!(buffer.get_cursor(), 11);


        // Hello Worl<d>
        buffer.go_to_prev_char();
        is_value_same_initial(&buffer);
        assert_eq!(buffer.get_cursor(), 10);


        // Hello Wo<r>ld
        buffer.go_to_prev_char();
        buffer.go_to_prev_char();
        is_value_same_initial(&buffer);
        assert_eq!(buffer.get_cursor(), 8);


        // Hello Wor<l>d
        buffer.go_to_next_char();
        is_value_same_initial(&buffer);
        assert_eq!(buffer.get_cursor(), 9);


        // Hello World<>
        for _ in 0..100 {
            buffer.go_to_next_char();
        }
        is_value_same_initial(&buffer);
        assert_eq!(buffer.get_cursor(), 11);


        // <H>ello World
        for _ in 0..100 {
            buffer.go_to_prev_char();
        }
        is_value_same_initial(&buffer);
        assert_eq!(buffer.get_cursor(), 0);


        // Hello World<>
        buffer.go_to_end();
        is_value_same_initial(&buffer);
        assert_eq!(buffer.get_cursor(), 11);


        // <H>ello Worl
        buffer.go_to_start();
        is_value_same_initial(&buffer);
        assert_eq!(buffer.get_cursor(), 0);
    }
}
