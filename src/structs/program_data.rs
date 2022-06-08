use std::collections::HashMap;

use super::var;
use super::func::{self, *};

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

    pub fn add_basic_functions(&mut self) {
        self.funcs.insert(format!("+"),      add::get_func());
        self.funcs.insert(format!("-"),      sub::get_func());
        self.funcs.insert(format!("*"),      mul::get_func());
        self.funcs.insert(format!("/"),      div::get_func());
        self.funcs.insert(format!("out"),    out::get_func());
        self.funcs.insert(format!("jumpnz"), jumpnz::get_func());
        self.funcs.insert(format!("jump"),   jump::get_func());
        self.funcs.insert(format!("debug"),  debug::get_func());
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
}
