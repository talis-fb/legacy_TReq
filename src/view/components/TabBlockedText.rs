use super::Component;
use crate::view::renderer::tui_rs::BackendTuiRs;
use crate::view::renderer::Tui;
use tui::layout::Rect;

pub struct TabBlockText {
    pub area: Rect,
    pub texts: Vec<(String, String)>,
    pub current: usize,
}
impl Component for TabBlockText {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let (_, current_content) = self.texts.get(self.current).unwrap();

        let titles_vec: Vec<&str> = self.texts.iter().map(|(title, _)| title.as_str()).collect();
        let mut title = String::new();

        for (i, content) in titles_vec.into_iter().enumerate() {
            if i == self.current {
                title.push_str(content.to_uppercase().as_str());
            } else {
                title.push_str(content);
            }

            if i < titles_vec.len() {
                title.push_str(" / ");
            }
        }

        f.render_text_in_block(&title, current_content, self.area);
    }
}
