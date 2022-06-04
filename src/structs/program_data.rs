use std::collections::HashMap;

use super::var;
use super::program_data;

#[derive(Clone)]
pub struct ProgramData {
    pub vars: HashMap<String, var::Var>,
    pub funcs: HashMap<String, program_data::ProgramData>,
    pub lables: HashMap<String, usize>,
    pub debug: bool,
}

impl ProgramData {
    pub fn new(enable_debug: bool) -> ProgramData {
        ProgramData {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            lables: HashMap::new(),
            debug: enable_debug,
        }
    }
}
