use crate::{base::doc::handler::DocReaderHandler, view::components::Component};
use tui::layout::Rect;

use super::style::{Color, Texts};

pub mod tui_rs;

pub trait Tui<T> {
    fn render_block_with_title_left(&mut self, title: Texts, area: T);
    fn render_block_with_title_center(&mut self, title: Texts, area: T);
    fn render_block_with_tab(&mut self, tabs: Vec<Texts>, current: usize, area: T);

    fn render_block_with_title_left_marked(&mut self, title: Texts, area: T);
    fn render_block_with_title_center_marked(&mut self, title: Texts, area: T);

    fn render_help_window<'a>(&mut self, doc_view: &'a DocReaderHandler, area: T);

    fn render_text<'a>(&mut self, text: Texts, area: T);
    fn render_text_raw<'a>(&mut self, text: &str, area: T);
    fn render_rows_texts<'a>(&mut self, text: Vec<Texts>, area: T);
    fn render_text_with_bg<'a>(&mut self, text: Texts, color: Color, area: T);

    fn render_bg_color<'a>(&mut self, color: Color, area: T);

    fn render_tablist(&mut self, tabs: Vec<Texts>, current: usize, area: T);
    fn render_tablist_marked(&mut self, tabs: Vec<Texts>, current: usize, area: T);

    fn render_divider_with_text(&mut self, text: Texts, area: T);

    fn render_clear_area(&mut self, area: T);
}

// TODO
pub trait Backend: Tui<Rect> {
    fn draw(&mut self, view: &dyn Component<Backend = Self>) -> ()
    where
        Self: Sized,
    {
        view.render(self)
    }
}
