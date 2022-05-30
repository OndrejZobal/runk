use crate::Var;
use crate::Assign;

pub struct Line<'a> {
    pub assignment: Option<Assign>,
    pub number: Var,
    pub file_name: &'a str,
    pub line_number: u64,
}
