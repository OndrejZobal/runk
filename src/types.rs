use std::collections::HashMap;
use std::fmt;

pub enum Num {
    N(u64),
    Z(i64),
}

impl Num {
    pub fn fit_into(&self, num: &mut Num) -> bool {
        match num {
            Num::Z(z1) => {
                match self {
                    Num::Z(z2) => {
                        *z1 = *z2;
                        return true;
                    },
                    Num::N(n2) => {
                        let converted = i64::try_from(*n2);
                        if !converted.is_ok() {
                            return false;
                        }

                        *z1 = converted.unwrap();
                        return true;
                    }
                }

            },

            Num::N(n1) => {
                match self {
                    Num::Z(z2) => {
                        let converted = u64::try_from(*z2);
                        if !converted.is_ok() {
                            return false;
                        }

                        *n1 = converted.unwrap();
                        return true;

                    },
                    Num::N(n2) => {
                        *n1 = *n2;
                        return true;
                    }
                }
            },
        }
    }
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Num::N(n) => {
                write!(f, "N({})", n)
            },
            Num::Z(z) => {
                write!(f, "Z({})", z)
            }
        }
    }
}

pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub enum Exp {
    Simple(Num),
    Funtion(Op, Vec<Exp>),
    MultiOp,
    Controll(Vec<String>),
    String(String),
}

pub enum Assign {
    Dec(
        Num, // Type
        String, // Name
    ),
    Nondec(String),
}

pub struct Line<'a> {
    pub assignment: Option<Assign>,
    pub number: Num,
    pub file_name: &'a str,
    pub line_number: u64,
}

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

pub struct ProgramData {
    pub vars: HashMap<String, Num>,
}

impl ProgramData {
    pub fn new() -> ProgramData {
        ProgramData {
            vars: HashMap::new(),
        }
    }
}
