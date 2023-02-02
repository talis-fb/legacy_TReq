use tui::layout::Rect;

use crate::view::components::Component;

pub trait Tui<T> {
    fn render_text(&mut self, text: &str, area: T);
    fn render_button(&mut self, label: &str, area: T);
}

pub trait Backend: Tui<Rect> {
    fn draw(&mut self, view: &dyn Component<Backend = Self>) -> ()
    where
        Self: Sized,
    {
        view.render(self)
    }
}

// #[derive(Default)]
// struct TextView {
//     text: String,
//     rect: Rect
// }
//
// impl Component for TextView {
//     fn render(&self, f: &mut impl Backend) {
//         f.render_text(&self.text, self.rect)
//     }
// }
