
// Operations
pub mod add;
pub mod sub;
pub mod mul;
pub mod div;
pub mod bin;
pub mod not;
pub mod and;
pub mod or;
pub mod go;
pub mod goif;
pub mod equal;
pub mod lesser;
pub mod r#in;
pub mod int;
pub mod nat;
pub mod greater;
pub mod lesser_equal;
pub mod greater_equal;
pub mod err;
pub mod cat;
pub mod cats;
pub mod line;
pub mod lines;
pub mod exit;

impl super::super::program_data::ProgramData {
    pub fn add_primitive_functions(&mut self) {
        self.funcs.insert(format!("+"),      add::get_func());
        self.funcs.insert(format!("-"),      sub::get_func());
        self.funcs.insert(format!("*"),      mul::get_func());
        self.funcs.insert(format!("/"),      div::get_func());
        self.funcs.insert(format!("<"),      lesser::get_func());
        self.funcs.insert(format!(">"),      greater::get_func());
        self.funcs.insert(format!("="),      equal::get_func());
        self.funcs.insert(format!("<="),     lesser_equal::get_func());
        self.funcs.insert(format!(">="),     greater_equal::get_func());
        self.funcs.insert(format!("in"),     r#in::get_func());
        self.funcs.insert(format!("int"),    int::get_func());
        self.funcs.insert(format!("nat"),    nat::get_func());
        self.funcs.insert(format!("bin"),    bin::get_func());
        self.funcs.insert(format!("not"),    not::get_func());
        self.funcs.insert(format!("and"),    and::get_func());
        self.funcs.insert(format!("or"),     or::get_func());
        self.funcs.insert(format!("go"),     go::get_func());
        self.funcs.insert(format!("goif"),   goif::get_func());
        self.funcs.insert(format!("err"),    err::get_func());
        self.funcs.insert(format!("cat"),    cat::get_func());
        self.funcs.insert(format!("cats"),   cats::get_func());
        self.funcs.insert(format!("line"),   line::get_func());
        self.funcs.insert(format!("lines"),  lines::get_func());
        self.funcs.insert(format!("exit"),   exit::get_func());

    }
}
