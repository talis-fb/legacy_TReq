use crate::app::App;

#[derive(Clone)]
pub struct InputBuffer {
    pub buffer: String,
    pub on_close: fn(&mut App, String),
}
impl InputBuffer {
    pub fn init() -> Self {
        InputBuffer {
            buffer: "".to_string(),
            on_close: |app: &mut App, s: String| {},
        }
    }
    pub fn append_char(&mut self, ch: char) {
        self.buffer.push(ch);
    }
    pub fn pop_char(&mut self) {
        self.buffer.pop();
    }
    pub fn set_callback(&mut self, callback: fn(&mut App, String)) {
        self.on_close = callback;
    }
    pub fn exec(&self, app: &mut App) {
        let callback = self.on_close;
        callback(app, self.buffer.clone())
    }
}
