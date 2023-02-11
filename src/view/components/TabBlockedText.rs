use super::Component;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use crate::view::style::Texts;
use tui::layout::Rect;

pub struct TabBlockText<'a> {
    pub area: Rect,
    pub texts: Vec<(&'a str, &'a str)>,
    pub current: usize,
}
impl Component for TabBlockText<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let (_, current_content) = self.texts.get(self.current).unwrap();

        let titles_vec: Vec<&str> = self.texts.iter().map(|(title, _)| *title).collect();
        let mut title = String::new();
        let titles_len = titles_vec.len();

        for (i, content) in titles_vec.into_iter().enumerate() {
            if i == self.current {
                title.push_str(content.to_uppercase().as_str());
            } else {
                title.push_str(content);
            }

            let last_index = titles_len - 1;
            if i < last_index {
                title.push_str(" / ");
            }
        }

        let title = Texts::from_str(&title);
        let current_content = Texts::from_str(current_content);

        f.render_text_in_block(title, current_content, self.area);
    }
}
