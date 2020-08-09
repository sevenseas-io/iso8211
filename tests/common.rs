use iso8211::Document;

pub fn assert_read(path: &str) -> Document {
    match Document::read(path) {
        Ok(d) => d,
        Err(e) => panic!(e),
    }
}
