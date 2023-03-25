use tui::text::Spans;

use crate::view::{help::DocView, style::Texts};

#[derive(Clone)]
pub struct DocReaderHandler {
    pub position: usize,
    pub doc: DocView,
}
impl DocReaderHandler {
    pub fn init(doc: DocView) -> Self {
        Self { doc, position: 0 }
    }
    pub fn get_doc_spans(&self) -> Vec<Spans> {
        self.doc.to_vec_spans().split_at(self.position).1.to_vec()
    }
    pub fn get_texts(&self) -> Vec<Texts> {
        self.doc.to_texts_spans().split_at(self.position).1.to_vec()
    }
    pub fn get_position(&self) -> usize {
        self.position
    }
}

// -----------------------------
// Position manipulation -------
// -----------------------------
impl DocReaderHandler {
    pub fn go_to_next_row(&mut self) {
        let last_row_index = self.doc.get_number_rows() - 1;
        self.position = std::cmp::min(last_row_index, self.position + 1);
    }

    pub fn go_to_prev_row(&mut self) {
        if self.position > usize::MIN {
            self.position -= 1;
        }
    }

    pub fn go_to_start(&mut self) {
        self.position = 0;
    }

    pub fn go_to_end(&mut self) {
        self.position = self.doc.get_number_rows() - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_goes_up_and_down_corretly() {
        let data = r#"
        {
            "content": [
                [ ["Row 1", null] ],
                [ ["Row 2", null] ],
                [ ["Row 3", null] ],
                [ ["Row 4", null] ],
                [ ["Row 5", null] ],
                [ ["Row 6", null] ],
                [ ["Row 7", null] ],
                [ ["Row 8", null] ]
            ]
        }"#;

        let mut handler = DocReaderHandler::init(DocView::from_string(data.to_string()));

        assert_eq!(handler.position, 0);

        handler.go_to_next_row();

        assert_eq!(handler.position, 1);

        handler.go_to_next_row();
        handler.go_to_next_row();

        assert_eq!(handler.position, 3);

        for _ in 0..100 {
            handler.go_to_next_row();
        }

        assert_eq!(handler.position, 7);

        handler.go_to_prev_row();

        assert_eq!(handler.position, 6);

        handler.go_to_prev_row();
        handler.go_to_prev_row();

        assert_eq!(handler.position, 4);

        for _ in 0..100 {
            handler.go_to_prev_row();
        }

        assert_eq!(handler.position, 0);
    }

    #[test]
    fn should_goes_start_and_end_corretly() {
        let data = r#"
        {
            "content": [
                [ ["Row 1", null] ],
                [ ["Row 2", null] ],
                [ ["Row 3", null] ],
                [ ["Row 4", null] ],
                [ ["Row 5", null] ],
                [ ["Row 6", null] ],
                [ ["Row 7", null] ],
                [ ["Row 8", null] ]
            ]
        }"#;

        let mut handler = DocReaderHandler::init(DocView::from_string(data.to_string()));

        handler.go_to_next_row();
        handler.go_to_next_row();

        handler.go_to_start();
        assert_eq!(handler.position, 0);

        handler.go_to_end();
        assert_eq!(handler.position, 7);
    }
}
