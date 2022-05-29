use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use colored::Colorize;

#[macro_use]
mod prints;
use prints::*;

mod types;
use types::*;

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
                "Z" => Num::Z(0),
                "N" => Num::N(0),
                &_  => {
                    syntax_error(filename, &line_number,
                                format!("Unknown type \"{}\"",
                                        input[0].string.italic()));
                    // This is unreachable
                    Num::Z(0)
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
        color_print!("Not Assigned: ", bright_yellow);
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
                Num::Z(z) => {
                    // TODO assing simple expression
                    data.vars.insert(string, num);
                },
                Num::N(n) => {
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
                Num::Z(z) => {
                    // TODO assing simple expression
                    data.vars.insert(string, Num::Z(0));
                },
                Num::N(n) => {
                    // TODO assing simple expression
                    data.vars.insert(string, Num::N(0));
                },
            }
        },
    }
}

fn resolve_simple_expression(word: &Word, file_name: &str, line_number: &u64) -> Num {
    let num = (&word.string).parse::<u64>();

    // At this point we are not sure what type this expression
    // needs to be, so we just put it in the one that fits bets
    // and if we later down the road need different types we
    // can just convert it.
    if num.is_ok() {
        return Num::N(num.unwrap());
    }

    let num = (&word.string).parse::<i64>();
    if num.is_ok() {
        return Num::Z(num.unwrap());
    }
    else {
        // Failed to parse as simple expression:
        syntax_error(file_name, &line_number,
                        format!("Invalid expression \"{}\"!", &word.string.italic())
        );
    }
    Num::N(0) // unreachable
}

fn resolve_op(word: &Word) -> Op {
    Op::Add
}

fn resolve_function_expression(input: &[Word], file_name: &str, line_number: &u64) -> Num {
    let mut op: Option<Op> = None;
    let mut operands: Vec<Num> = Vec::new();
    for (i, word) in input.iter().enumerate() {
        // First operand in a function is the operation.
        if i == 0 {
            op = Some(resolve_op(&word));
            continue;
        }

        operands.push(resolve_exp(&input[i..], file_name, line_number));
    }
    Num::N(0) // unreachable
}

fn resolve_exp(input: &[Word], file_name: &str, line_number: &u64) -> Num {
    // TODO Loop useless??
    for (i, word) in input.iter().enumerate() {
        // On first iteration we can determine, what kind of
        // expression is this.
        if i == 0 {
            // Simple expression
            if input.len() == 1 {
                return resolve_simple_expression(word, file_name, line_number);
            }

            // Function expression
            if word.string == "(" {
                todo!();
                return resolve_function_expression(&input[i+1..], file_name, line_number);
            }
        }
    }

    // If nothing was returned yet...
    syntax_error(file_name, &line_number,
                    format!("All hell broke loose when processing an expression!")
    );
    Num::N(0) // unreachable
}


fn parse_line<'a>(input: &'a Vec::<Word>, filename: &'a str, line_number: u64) -> Line<'a> {

    for (i, word) in input.iter().enumerate() {
        if i == 0 {
            eprint!("{}", format!("RUN {}\t| ", word.line).yellow());
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
    let mut expression_start_index: usize = 0;

    // Look for the asssignment operator ":"
    for (i, word) in input.iter().enumerate() {
        if word.string == ":" {
            assign = parse_assignment(&input[..i], filename, line_number);
            expression_start_index = i+1;
            break;
        }
    }

    // Check if expression is missing.
    if input.len() <= expression_start_index {
        syntax_error(filename, &line_number, "Missing expression!".to_string());
    }

    // Parsing expression
    let num: Num = resolve_exp(&input[expression_start_index..], &&filename, &line_number);

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
                let parsed_line = parse_line(&line, &file_name, u64::try_from(i).unwrap()+1);
                execute_line(parsed_line, data);
                line = Vec::<Word>::new();
            }
        }
    }
}

fn print_banner() {
    color_print!("=======================================\n", bright_blue);
    color_print!("Ridiculously Useless Numeric Katastrophe", blue);
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

    eprintln!("\nVariables:");
    for (key, val) in program_data.vars.iter() {
        eprintln!("{}: {}", key, val);
    }
}
