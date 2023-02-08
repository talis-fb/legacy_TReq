use tui::layout::Rect;
use crate::{view::components::Component, base::doc::handler::DocReaderHandler};

pub mod tui_rs;

pub trait Tui<T> {
    fn render_block_with_title_left(&mut self, title: &str, area: T);
    fn render_block_with_title_center(&mut self, title: &str, area: T);
    fn render_block_with_tab(&mut self, tabs: Vec<&str>, current: usize, area: T);

    fn render_help_window<'a>(&mut self, doc_view: &'a DocReaderHandler, area: T);

    fn render_text<'a>(&mut self, text: &str, area: T);
    fn render_text_in_block<'a>(&mut self, block_title: &str, text: &str, area: Rect);
    fn render_text_with_bg<'a>(&mut self, text: &str, area: Rect);

    fn render_tablist(&mut self, tabs: Vec<&str>, current: usize, area: T);
}

pub trait Backend: Tui<Rect> {
    fn draw(&mut self, view: &dyn Component<Backend = Self>) -> ()
    where
        Self: Sized,
    {
        view.render(self)
    }
}
