use tui::text::Spans;

use crate::view::help::DocView;


#[derive(Clone)]
pub struct DocReaderHandler {
    pub position: usize,
    pub doc: DocView
}
impl DocReaderHandler {
    pub fn init(doc: DocView) -> Self {
        Self {
            doc,
            position: 0,
        }
    }
    pub fn get_doc_spans(&self) -> Vec<Spans> {
        self.doc.to_vec_spans().split_at(self.position).1.to_vec()
    }
    pub fn get_position(&self) -> usize {
        self.position
    }
    pub fn goto_new_pos(&mut self, pos:usize) {
        self.position = pos;
    }
}
