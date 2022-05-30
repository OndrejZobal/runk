use std::collections::HashMap;

use crate::Var;

pub struct ProgramData {
    pub vars: HashMap<String, Var>,
}

impl ProgramData {
    pub fn new() -> ProgramData {
        ProgramData {
            vars: HashMap::new(),
        }
    }
}
