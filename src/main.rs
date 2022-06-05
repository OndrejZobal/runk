use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

use colored::Colorize;
use structs::{program_data};

//use clap::Parser;

mod runk;
mod structs;
mod expressions;
#[macro_use]
mod prints;

fn main() {

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

    let input_file_reader: Box<dyn BufRead> = match input_file_name {
        None => Box::new(BufReader::new(io::stdin())),
        Some(ref filename) => {
            let file = File::open(&filename);
            match file {
                Ok(f) => Box::new(BufReader::new(f)),
                Err(e) => {
                    color_print!("Error: ", red bold);
                    eprintln!("cannot read file \"{}\": {}", filename.italic(), e);
                    std::process::exit(1);
                }
            }
        }
    };

    let mut program_data = program_data::ProgramData::new(debug.clone());

    runk::run_runk_buffer(input_file_reader,
                 match &input_file_name {
                     None => "<stdin>",
                     Some(file) => &file[..],
                 },
                 &mut program_data,
    );
}
