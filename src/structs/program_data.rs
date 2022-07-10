use std::collections::HashMap;

use super::var;
use super::func::{self, *};

const VAR_ERROR: &str = "ERROR";

#[derive(Clone)]
pub struct ProgramData {
    /// Stores runk program's variables
    pub vars: HashMap<String, var::Var>,
    /// Stores runk program's functions. Currently only stores primitive functions.
    pub funcs: HashMap<String, func::Func>,
    /// Stores lables of a runk program. For the purpouse of jumping to them.
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

    pub fn add_special_variables(&mut self) {
        self.vars.insert(VAR_ERROR.to_string(), var::Var::t(format!("")).unwrap());
    }

    pub fn debug_vars_print(&self) {
        crate::color_print!("\nVariables:\n", blue italic);
        for (key, val) in self.vars.iter() {
            eprintln!("{}: {}", key, val);
        }
    }

    pub fn debug_funcs_print(&self) {
        crate::color_print!("\nFunctions:\n", blue italic);
        for (key, _val) in self.funcs.iter() {
            eprintln!("{}", key);
        }
    }

    pub fn debug_status(&self) {
        self.debug_vars_print();
        self.debug_funcs_print();
    }

    pub fn set_error(&mut self, string: String) -> String {
        let old_string = self.vars.get(VAR_ERROR).unwrap().clone();
        self.vars.insert(self::VAR_ERROR.to_string(), var::Var::t(string).unwrap());

        if let var::Var::T(text) = old_string {
            return text.clone();
        }
        panic!("VAR_ERROR was not initialized properly!");
    }
}
