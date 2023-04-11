use self::handler::DocReaderHandler;
use crate::view::help::DocView;
// use std::path::Path;
pub mod handler;

pub struct DocsFactory;
impl DocsFactory {
    // fn get_file_content(file: &str) -> String {
    //     let path_file = Path::new(".").join("doc").join(file);
    //     std::fs::read_to_string(path_file)
    //         .map_err(|e| e.to_string())
    //         .unwrap()
    // }

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
