use super::word;
use std::fmt;

#[derive(Debug)]
pub struct Line<'a> {
    pub file_name: &'a str,
    pub line_number: usize,
    pub content: Vec<word::Word>,
    pub original: String,
    pub is_last: bool,
}

impl Line<'_> {
    pub fn new<'a>(file_name: &'a str, line_number: usize, is_last: bool) -> Line<'a> {
        Line {
            file_name: file_name,
            line_number: line_number,
            content: Vec::new(),
            original: String::new(),
            is_last: is_last,
        }
    }
}

impl fmt::Display for Line<'_> {
    fn fmt<'a>(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = String::new();
        for word in &self.content {
            string.push_str(&word.original);
            string.push_str(" ")
        }
        write!(f, "{}", string)
    }
}
