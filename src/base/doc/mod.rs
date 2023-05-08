use self::handler::DocReaderHandler;
use crate::view::help::DocView;
// use std::path::Path;
pub mod handler;

pub struct DocsFactory;
impl DocsFactory {
    pub fn help_reader() -> DocReaderHandler {
        let content = include_str!("views/help.json").to_string();
        DocReaderHandler::init(DocView::from_string(content))
    }

    pub fn welcome_reader() -> DocReaderHandler {
        let content = include_str!("views/welcome.json").to_string();
        DocReaderHandler::init(DocView::from_string(content))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_if_views_files_is_valid() {
        DocsFactory::help_reader();
        DocsFactory::welcome_reader();
    }
}
