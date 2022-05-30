use std::fmt;

pub struct Word {
    pub string: String,
    pub column: u64,
    pub line: u64,
}

impl Word {
    pub fn clone(&self) -> Word {
        Word { string: self.string.clone(), column: self.column, line: self.line }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.string)
    }
}

impl fmt::Debug for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.string)
    }
}
