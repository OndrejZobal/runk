
// Operations
pub mod add;
pub mod sub;
pub mod mul;
pub mod div;
pub mod out;
pub mod jump;
pub mod jumpnz;
pub mod lesser;
pub mod greater;
pub mod lesser_equal;
pub mod greater_equal;

impl super::super::program_data::ProgramData {
    pub fn add_primitive_functions(&mut self) {
        self.funcs.insert(format!("+"),      add::get_func());
        self.funcs.insert(format!("-"),      sub::get_func());
        self.funcs.insert(format!("*"),      mul::get_func());
        self.funcs.insert(format!("/"),      div::get_func());
        self.funcs.insert(format!("<"),      lesser::get_func());
        self.funcs.insert(format!(">"),      greater::get_func());
        self.funcs.insert(format!("<="),     lesser_equal::get_func());
        self.funcs.insert(format!(">="),     greater_equal::get_func());
        self.funcs.insert(format!("out"),    out::get_func());
        self.funcs.insert(format!("jump"),   jump::get_func());
        self.funcs.insert(format!("jumpnz"), jumpnz::get_func());
    }
}
