use std::collections::HashMap;

use super::var::Var;

#[derive(Clone)]
pub struct ProgramData {
    pub vars: HashMap<String, Var>,
    pub debug: bool,
}

impl ProgramData {
    pub fn new(enable_debug: bool) -> ProgramData {
        ProgramData {
            vars: HashMap::new(),
            debug: enable_debug,
        }
    }
}
