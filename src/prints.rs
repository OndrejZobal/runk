pub use colored::Colorize;
use std::process;
use crate::structs::word;

use crate::structs::source_info;

#[macro_export]
macro_rules! color_print {
    ( $string:expr, $($color:ident) + ) => {
        use colored::Colorize;
        eprint!("{}", format!($string)$(.$color())+);
        // FIXME io::stderr().flush();
    }
}

pub fn fatal_error(info: &source_info::SourceInfo, message: String, opt_word: Option<&word::Word>) -> ! {

    let red_border = "\t| ".to_string().red();

    color_print!("\nFatal Error: ", red bold);
    eprintln!("{}:{}: {}", info.file_name.bold(), info.line_number.to_string().bold(), message);


    if opt_word.is_some() {
        eprintln!("{}", &red_border);
    }

    eprintln!("{}{}{}", &info.line_number.to_string().red(), &red_border, &info.original);

    if opt_word.is_some() {
        let word = opt_word.unwrap();
        eprint!("{}", &red_border);
        for _i in 0..word.column {
            eprint!(" ");
        }
        for _i in 0..word.original.len() {
            eprint!("{}", "^".to_string().bright_red());
        }
        eprintln!("");
    }

    process::exit(1);

}
