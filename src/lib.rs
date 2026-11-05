#[derive(Debug, Clone)]
pub struct DocumentState {
    pub file_path: String,
    pub current_page: u32,
    pub total_pages: u32,
}

impl DocumentState {
    pub fn new(path: &str, pages: u32) -> Self {
        Self { file_path: path.to_string(), current_page: 1, total_pages: pages }
    }
    pub fn next_page(&mut self) {
        if self.current_page < self.total_pages { self.current_page += 1; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_document_reader_navigation() {
        let mut doc = DocumentState::new("/tmp/sample.pdf", 10);
        assert_eq!(doc.current_page, 1);
        doc.next_page();
        assert_eq!(doc.current_page, 2);
    }
}
