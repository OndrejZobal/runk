use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::process;
use colored::Colorize;

#[cfg(target_family = "unix")]
use isatty::stdin_isatty;

use runk::structs::program_data;

fn platform_eof_key() -> String {
        #[cfg(target_family = "unix")]
        return format!("d");
        #[cfg(target_family = "windows")]
        return format!("c");
    }

fn print_repl_banner() {
    eprintln!("\
{} version {} - {},
Copyright {}

 {} is an interpreter and a programming language visit
 https://github.com/OndrejZobal/runk for more instructions.

 This is Free Software licensed under GNU GPL-3.0 or any later version.
 No warranty is provided, failures are guaranteed!

 To signal end of file press {}.

 Long live Ronald!\
",
            format!("{}", env!("CARGO_PKG_NAME")).bold(),
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_AUTHORS"),
            env!("CARGO_PKG_NAME"),
            format!("Ctrl+{}", platform_eof_key()).bold(),
    );
}

fn main() {
    // Try to enable pretty colors on Windows.
    #[cfg(target_family = "windows")]
    match enable_ansi_support::enable_ansi_support() {
        Ok(_) => {},
        Err(_) => {},
    };

    let mut args: Vec<_> = env::args().collect();
    let mut debug = false;

    for (i, arg) in args.iter().enumerate() {
        if arg == "--debug" && i != 0 {
            debug = true;
            args.remove(i);
            break;
        }
    }

    let input_file_name = args.iter().nth(1);

    let mut is_input_stdin = false;
    let input_file_reader: Box<dyn BufRead> = match input_file_name {
        None => {
            is_input_stdin = true;
            Box::new(BufReader::new(io::stdin()))
        },
        Some(ref filename) => {
            let file = File::open(&filename);
            match file {
                Ok(f) => Box::new(BufReader::new(f)),
                Err(e) => {
                    runk::color_print!("Error: ", red bold);
                    eprintln!("cannot read file \"{}\": {}", filename.italic(), e);
                    std::process::exit(1);
                }
            }
        }
    };

    let mut program_data = program_data::ProgramData::new(debug.clone());
    let mut repl_mode = false;
    #[cfg(target_family = "unix")]
    {repl_mode = is_input_stdin && stdin_isatty();}

    if repl_mode {
        print_repl_banner();
    }

    runk::run_runk_buffer(input_file_reader,
                 match &input_file_name {
                     None => "<stdin>",
                     Some(file) => &file[..],
                 },
                 &mut program_data,
                 repl_mode,

    );
}
