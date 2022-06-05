pub use colored::Colorize;
use std::process;

use crate::structs::source_info;

#[macro_export]
macro_rules! color_print {
    ( $string:expr, $($color:ident) + ) => {
        use colored::Colorize;
        eprint!("{}", format!($string)$(.$color())+);
        // FIXME io::stderr().flush();
    }
}

pub fn syntax_error(info: &source_info::SourceInfo, message: String) -> ! {
    color_print!("Syntax Error: ", red bold);
    eprintln!("{}:{}: {}", info.file_name.bold(), info.line_number.to_string().bold(), message);
    process::exit(1);

}

pub fn runtime_error(info: &source_info::SourceInfo, message: String) -> ! {
    color_print!("Runtime Error: ", red bold);
    eprintln!("{}:{}: {}", info.file_name.bold(), info.line_number.to_string().bold(), message);
    process::exit(1);

}
