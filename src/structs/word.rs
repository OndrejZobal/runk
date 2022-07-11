use std::fmt;
use crate::parser::rtoken;
use super::line;

#[derive(Clone)]
pub struct Word {
    /// Runk token this word represents.
    pub rtoken: rtoken::Rtoken,
    /// The original word as it appears in the file.
    pub original: String,
    /// Actual column in the file
    pub column: usize,
    /// Actual line number in the file.
    pub line: usize,
    /// Line number in the tokenized vector of words.
    pub parsed_line: usize,
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", &self.rtoken, &self.original)
    }
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({})", &self.rtoken, &self.original)
    }
}
