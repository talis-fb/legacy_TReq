use super::BlockText::BlockText;
use super::Component;
use crate::view::renderer::tui_rs::BackendTuiRs;

use crate::view::style::{Color, Style, Text, Texts};
use tui::layout::Rect;

pub struct TabBlockText<'a> {
    pub area: Rect,
    pub texts: Vec<(&'a str, &'a str)>,
    pub current: usize,
    pub marked: bool,
}
impl Component for TabBlockText<'_> {
    type Backend = BackendTuiRs;
    fn render(&self, f: &mut Self::Backend) {
        let (_, current_content) = self.texts.get(self.current).unwrap();

        let titles_vec: Vec<&str> = self.texts.iter().map(|f| f.0).collect();
        let titles_len = titles_vec.len();
        let mut titles_texts_with_style: Vec<(String, bool)> = vec![];

        for (i, content) in titles_vec.into_iter().enumerate() {
            if i == self.current {
                titles_texts_with_style.push((content.to_uppercase(), true));
            } else {
                titles_texts_with_style.push((content.to_string(), false));
            }

            let last_index = titles_len - 1;
            if i < last_index {
                titles_texts_with_style.push((" / ".to_string(), false));
            }
        }

        let title = Texts {
            body: titles_texts_with_style
                .iter()
                .map(|(body, is_styled)| Text {
                    body,
                    style: if *is_styled {
                        Some(Style {
                            color: Color::Yellow,
                            property: None,
                        })
                    } else {
                        None
                    },
                })
                .collect(),
        };

        let current_content = Texts::from_str(current_content);

        let component = BlockText {
            area: self.area,
            title,
            content: current_content,
            marked: self.marked,
        };

        component.render(f);

        // f.render_block_with_title_left(title, self.area);
        // f.render_text(current_content, self.area);
    }
}
