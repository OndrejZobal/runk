use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

use colored::Colorize;
use structs::{program_data};

use clap::Parser;

mod runk;
mod structs;
mod expressions;
#[macro_use]
mod prints;

fn main() {
    let input_file_name = env::args().nth(1);

    let input_file_reader: Box<dyn BufRead> = match input_file_name {
        None => Box::new(BufReader::new(io::stdin())),
        Some(ref filename) => Box::new(BufReader::new(File::open(&filename).unwrap()))
    };

    let mut program_data = program_data::ProgramData::new(false);

    runk::run_runk_buffer(input_file_reader,
                 match &input_file_name {
                     None => "<stdin>",
                     Some(file) => &file[..],
                 },
                 &mut program_data,
    );
}
