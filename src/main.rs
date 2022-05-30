use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use colored::Colorize;

#[macro_use]
mod prints;
use prints::*;

#[path = "structs/mod.rs"]
mod structs;
use structs::{var::Var, assign::Assign, program_data::ProgramData, word::Word, line::Line};

mod expressions;
use expressions::resolve_exp;

fn parse_assignment(input: &[Word], filename: &str, line_number: u64) -> Option<Assign> {
    // Nondeclarative
    if input.len() == 1 {
        if input[0].string.parse::<f64>().is_ok() {
            syntax_error(filename, &line_number,
                            format!("Variable name \"{}\" is invalid!",
                                    input[0].string.italic()))
        }

        return Some(Assign::Nondec(input[0].string.clone()));
    }

    // Declarative
    if input.len() == 2 {
        if input[1].string.parse::<f64>().is_ok() {
            syntax_error(filename, &line_number,
                            format!("Variable name \"{}\" is invalid!",
                                    input[1].string.italic()))
        }
        return Some(Assign::Dec(
            match &input[0].string[..] {
                "Z" => Var::Z(0),
                "N" => Var::N(0),
                &_  => {
                    syntax_error(filename, &line_number,
                                format!("Unknown type \"{}\"",
                                        input[0].string.italic()));
                    // This is unreachable
                    Var::Z(0)
                }
            },
            input[1].string.clone()
        ));
    }

    return None;

}

fn execute_line(line: Line, data: &mut ProgramData) {
    // Processing assignment
    if line.assignment.is_none() {
        color_print!("Not Assigned: ", yellow);
        eprintln!("{}", line.number);
        return;
    }

    match line.assignment.unwrap() {
        Assign::Dec(mut num, string) => {
            if data.vars.contains_key(&string) {
                runtime_error(line.file_name, &line.line_number,
                              format!("Redefinition of variable \"{}\"!",
                                      string.italic())
                );
            }

            // Copying the acutal number to "num"
            if !line.number.fit_into(&mut num) {
                runtime_error(line.file_name, &line.line_number,
                              format!("Failiure while converting numbers during assignment.")
                );
            }

            // New variable is beeing defined. We match agains the type specified
            // in 'line'
            match num {
                Var::Z(z) => {
                    // TODO assing simple expression
                    data.vars.insert(string, num);
                },
                Var::N(n) => {
                    // TODO assing simple expression
                    data.vars.insert(string, num);
                },
            }
        },
        Assign::Nondec(string) => {
            if !data.vars.contains_key(&string) {
                runtime_error(line.file_name, &line.line_number,
                              format!("Variable \"{}\" assigned before definition!",
                                      string.italic())
                );
            }

            // Modifing existing variable, need to maintain it's previous type.
            match data.vars.get(&string).unwrap() {
                Var::Z(z) => {
                    // TODO assing simple expression
                    data.vars.insert(string, Var::Z(0));
                },
                Var::N(n) => {
                    // TODO assing simple expression
                    data.vars.insert(string, Var::N(0));
                },
            }
        },
    }
}


fn parse_line<'a>(input: &'a Vec::<Word>, filename: &'a str, line_number: u64, data: &ProgramData) -> Line<'a> {

    for (i, word) in input.iter().enumerate() {
        if i == 0 {
            eprint!("{}", format!("RUN {}\t| ", word.line).bright_yellow());
        }
        eprint!("{} ", word.string);
    }
    eprintln!("");

    // First word can either be:
    // A type (Is Z, N or R)
    // A variable name (is a string)
    // A simple expresion (is a number)
    // Other expression (has parethesees)

    // Parsing assignment
    let mut assign: Option<Assign> = None;
    let mut exp_start_index: usize = 0;

    // Look for the asssignment operator ":"
    for (i, word) in input.iter().enumerate() {
        if word.string == ":" {
            assign = parse_assignment(&input[..i], filename, line_number);
            exp_start_index = i+1;
            break;
        }
    }

    // Check if expression is missing.
    if input.len() <= exp_start_index {
        syntax_error(filename, &line_number, "Missing expression!".to_string());
    }

    // Parsing expression
    let (num, exp_end_index) = resolve_exp(&input[exp_start_index..], &filename, &line_number, data);

    if exp_start_index + exp_end_index != input.len() {
        syntax_error(filename, &line_number, format!("Unexpected text after expression!"));
    }


    let line = Line { assignment: assign, number: num, file_name: filename, line_number: line_number };
    line
}

fn is_special_operator(c: &char) -> bool {
    "()[]<>=:!".chars().any(|s| s.eq(c))
}


fn process_file(input_file_reader: Box<dyn BufRead>, file_name: String, data: &mut ProgramData) {
    let mut line = Vec::<Word>::new();
    let mut word: Option<Word> = None;
    let mut depth: u64 = 0;

    macro_rules! push_word {
        () => {
            match &mut word {
                None => {},
                Some(w) => {
                    if w.string.len() != 0 {
                        line.push(w.clone());
                        word = None;
                    }
                },
            }
        }
    }

    for (i, _line) in input_file_reader.lines().enumerate() {
        let mut escape_char = false;
        for (j, c) in _line.unwrap().chars().enumerate() {
            if !escape_char {
                if c == '#' {
                    push_word!();
                    break;
                }
                else if c.is_whitespace() {
                    push_word!();
                    continue;
                }
                else if c == '\\' {
                    escape_char = true;
                    continue;
                }
                else if is_special_operator(&c) {
                    push_word!();
                }
            }

            // Pushing char
            match word {
                None => {
                    word = Some(Word {
                        string: c.to_string(),
                        column: u64::try_from(j+1).unwrap(),
                        line: u64::try_from(i+1).unwrap()}
                    );
                },
                Some(ref mut w) => {
                    w.string.push(c);
                },
            }

            if !escape_char {
                if c == '(' || c == '[' || c == '<' {
                    depth += 1;
                    push_word!();
                }
                else if c == ')' || c == ']' || c == '>' {
                    depth -= 1;
                    push_word!();
                }
            }
            else {
                escape_char = false;
                continue;
            }
        }
        push_word!();
        if depth == 0 {
            if line.len() != 0 {
                let parsed_line = parse_line(&line, &file_name, u64::try_from(i).unwrap()+1, &data);
                execute_line(parsed_line, data);
                line = Vec::<Word>::new();
            }
        }
    }
}

fn print_banner() {
    color_print!("=======================================\n", bright_blue);
    color_print!("Ridiculously Useless Vareric Katastrophe", blue);
    color_print!("========================================\n", bright_blue);
}

fn main() {
    //print_banner();

    let input_file_name = env::args().nth(1);

    let input_file_reader: Box<dyn BufRead> = match input_file_name {
        None => Box::new(BufReader::new(io::stdin())),
        Some(ref filename) => Box::new(BufReader::new(File::open(&filename).unwrap()))
    };

    let mut program_data = ProgramData::new();

    process_file(input_file_reader,
                 match input_file_name {
                     None => String::from("<stdin>"),
                     Some(file) => file
                 },
                 &mut program_data,
    );

    eprintln!("{}", "DONE".green());

    color_print!("\nVariables:\n", blue italic);
    for (key, val) in program_data.vars.iter() {
        eprintln!("{}: {}", key, val);
    }
}
