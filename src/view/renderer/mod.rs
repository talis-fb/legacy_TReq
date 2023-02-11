use tui::layout::Rect;
use crate::{view::components::Component, base::doc::handler::DocReaderHandler};

use super::style::{Texts, Color};

pub mod tui_rs;

pub trait Tui<T> {
    fn render_block_with_title_left(&mut self, title: Texts, area: T);
    fn render_block_with_title_center(&mut self, title: Texts, area: T);
    fn render_block_with_tab(&mut self, tabs: Vec<Texts>, current: usize, area: T);

    fn render_help_window<'a>(&mut self, doc_view: &'a DocReaderHandler, area: T);

    fn render_text<'a>(&mut self, text: Texts, area: T);
    fn render_text_in_block<'a>(&mut self, block_title: Texts, text: Texts, area: Rect);
    fn render_text_with_bg<'a>(&mut self, text: Texts, color: Color, area: Rect);

    fn render_tablist(&mut self, tabs: Vec<Texts>, current: usize, area: T);

    fn render_divider_with_text(&mut self, text: Texts, area: T);
}

pub trait Backend: Tui<Rect> {
    fn draw(&mut self, view: &dyn Component<Backend = Self>) -> ()
    where
        Self: Sized,
    {
        view.render(self)
    }
}
