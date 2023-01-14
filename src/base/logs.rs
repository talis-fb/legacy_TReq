#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogType {
    Error,
    Warning,
    Help,

    Empty,
    InputMode,
}
impl Default for LogType {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Default, Clone)]
pub struct Log {
    pub log_type: LogType,
    pub title: String,
    pub detail: Option<String>,
}

impl Log {
    pub fn with_type(&self, t: LogType) -> Self {
        let mut log = self.clone();
        log.log_type = t;
        log
    }
    pub fn with_title(&self, t: String) -> Self {
        let mut log = self.clone();
        log.title = t;
        log
    }
    pub fn with_detail(&self, t: String) -> Self {
        let mut log = self.clone();
        log.detail = Some(t);
        log
    }
}
