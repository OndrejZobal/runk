use colored::Colorize;
use std::process;

macro_rules! color_print {
    ( $string:expr, $($color:ident) + ) => {
        eprint!("{}", format!($string)$(.$color())+);
        // FIXME io::stderr().flush();
    }
}

pub fn syntax_error(filename: &str, line_number: &u64, message: String) {
    color_print!("Syntax Error: ", red bold);
    eprintln!("{}:{}: {}", filename.bold(), line_number.to_string().bold(), message);
    process::exit(1);

}

pub fn runtime_error(filename: &str, line_number: &u64, message: String) {
    color_print!("Runtime Error: ", red bold);
    eprintln!("{}:{}: {}", filename.bold(), line_number.to_string().bold(), message);
    process::exit(1);

}
