use std::fmt;
use crate::parser::rtoken;

#[derive(Clone)]
pub struct Word {
    pub rtoken: rtoken::Rtoken,
    pub original: String,
    pub column: u64,
    pub line: u64,
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
