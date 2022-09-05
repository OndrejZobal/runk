#[derive(Clone)]
pub struct SourceInfo<'a> {
    pub line_number: usize,
    pub file_name:   &'a str,
    pub original:    String,
}

impl SourceInfo<'_> {
    pub fn new<'a>(line_number: usize, file_name: &'a str, original: String) -> SourceInfo<'a> {
        SourceInfo {
            line_number: line_number,
            file_name: file_name,
            original,
        }
    }
}
