use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use colored::Colorize;

use crate::structs::{var, assign, program_data, word, source_info, line};
use crate::prints::{syntax_error, runtime_error};
use crate::expressions::resolve_exp;
use crate::color_print;

/// Parses assignment (if it exsits)
fn parse_assignment(input: &[word::Word], info: &source_info::SourceInfo) -> (Option<assign::Assign>, usize) {
    // Parsing assignment
    let mut assign: Option<assign::Assign> = None;
    let mut exp_start_index: usize = 0;

    // Look for the asssignment operator ":"
    for (i, word) in input.iter().enumerate() {
        if word.string == ":" {
            exp_start_index = i;
            break;
        }
    }

    // Check if expression is missing.
    if input.len() <= exp_start_index {
        syntax_error(&info, "Missing expression!".to_string());
    }

    // This assumes that its cropeed lol

    // Nondeclarative
    if exp_start_index == 1 {
        if input[0].string.parse::<f64>().is_ok() {
            syntax_error(&info, format!("Variable name \"{}\" is invalid!", input[0].string.italic()))
        }

        return (
            Some(assign::Assign::Nondec(input[0].string.clone())),
            exp_start_index+1
        );
    }

    // Declarative
    if exp_start_index == 2 {
        if input[1].string.parse::<f64>().is_ok() {
            syntax_error(&info, format!("Variable name \"{}\" is invalid!", input[1].string.italic()))
        }
        return (
            Some(assign::Assign::Dec(
            match &input[0].string[..] {
                "Z" => var::Var::Z(0),
                "N" => var::Var::N(0),
                &_  => {
                    syntax_error(&info, format!("Unknown type \"{}\"", input[0].string.italic()));
                }
            },
            input[1].string.clone()
            )),
            exp_start_index+1
        );
    }

    return (None, exp_start_index);
}

/// Executes the tokenized line and assignment
///
/// ## Arguments
/// line The tokenized line
/// data Internal data of the program
fn execute_asignment(assign: &Option<assign::Assign>,
                     vale: &var::Var,
                     info: &source_info::SourceInfo,
                     data: &mut program_data::ProgramData) -> Option<usize> {
    // Processing assignment
    if assign.is_none() {
        if data.debug {
            color_print!("Not Assigned: ", yellow);
            eprintln!("{}", info.line_number);
        }
        return None;
    }

    let bruh = assign.as_ref().unwrap().clone();
    match bruh {
        assign::Assign::Dec(mut num, ref string) => {
            if data.vars.contains_key(&string[..]) {
                runtime_error(&info, format!("Redefinition of variable \"{}\"!", string.italic())
                );
            }

            // Copying the acutal number to "num"
            if !vale.fit_into(&mut num) {
                runtime_error(&info, format!("Failiure while converting numbers during assignment."));
            }

            // New variable is beeing defined. We match agains the type specified
            // in 'line'
            match num {
                var::Var::Z(z) => {
                    // TODO assing simple expression
                    data.vars.insert((&string[..]).to_string(), num);
                },
                var::Var::N(n) => {
                    // TODO assing simple expression
                    data.vars.insert((&string[..]).to_string(), num);
                },
            };
        },
        assign::Assign::Nondec(string) => {
            if !data.vars.contains_key(&string[..]) {
                runtime_error(&info, format!("Variable \"{}\" assigned before definition!", string.italic()));
            }

            // Modifing existing variable, need to maintain it's previous type.
            match data.vars.get(&string[..]).unwrap() {
                var::Var::Z(z) => {
                    // TODO assing simple expression
                    data.vars.insert((&string[..]).to_string(), var::Var::Z(0));
                },
                var::Var::N(n) => {
                    // TODO assing simple expression
                    data.vars.insert((&string[..]).to_string(), var::Var::N(0));
                },
            };
        },
    }

    None
}

/// Internal function for parsing a single (assignment &) expression.
/// It returns a `Line` struct that the makes things easier to work with.
/// Gets called by the run_file function.
///
/// ## Arguments
/// input Vector of tokens as they were found in the input file.
/// filename Name of the file beeing executed. Used for logging only.
/// line_number Line number of the beggingin of the expression.
/// data Internal state of the runk program.
// fn parse_line<'a>(input: Vec::<word::Word>, file_name: &'a str, line_number: usize) -> line::Line<'a> {
//
//     for (i, word) in input.iter().enumerate() {
//         if i == 0 {
//             eprint!("{}", format!("RUN {}\t| ", word.line).bright_yellow());
//         }
//         eprint!("{} ", word.string);
//     }
//     eprintln!("");
//
//     // First word can either be:
//     // A type (Is Z, N or R)
//     // A variable name (is a string)
//     // A simple expresion (is a number)
//     // Other expression (has parethesees)
//
//     // Parsing assignment
//     let mut assign: Option<assign::Assign> = None;
//     let mut exp_start_index: usize = 0;
//
//     // Look for the asssignment operator ":"
//     for (i, word) in input.iter().enumerate() {
//         if word.string == ":" {
//             assign = parse_assignment(&input[..i], &file_name, &line_number);
//             exp_start_index = i+1;
//             break;
//         }
//     }
//
//     // Check if expression is missing.
//     if input.len() <= exp_start_index {
//         syntax_error(file_name, &line_number, "Missing expression!".to_string());
//     }
//
//     // Parsing expression
//     let (num, exp_end_index) = resolve_exp(&input[exp_start_index..], &file_name, &line_number, data);
//
//     if exp_start_index + exp_end_index != input.len() {
//         syntax_error(file_name, &line_number, format!("Unexpected text after expression!"));
//     }
//
//
//     let line = line::Line { assignment: assign, number: num,
//                             info: line::source_info::SourceInfo::new(&usize::try_from(line_number).unwrap(), &file_name, &file_name) };
//     line
// }

fn is_special_operator(c: &char) -> bool {
    "()[]<>=:!".chars().any(|s| s.eq(c))
}


/// Executes runk code in given buffer. This is the main way of executing runk code
///
/// ## Description
/// This function mainly collects individual (assignment &) expression, separates
/// individual tokens and puts them into a vec and calls other functions for further processing one by one.
///
/// ## Arguments
/// input_file_reader input buffer containing the runk code that will get executed.
/// file_name Name of the original file that is beeing executed. It is only used for logging purpouses.
/// data Stores internal state of the runk program like Variables etc...
/// TODO Consider returining a result.
pub fn parse_file<'a>(input_file_reader: Box<dyn BufRead>, info: &source_info::SourceInfo, file_name: &'a str) -> Vec::<line::Line<'a>> {
    let mut lines = Vec::<line::Line>::new();
    let mut line = line::Line::new(&file_name, usize::MAX);
    let mut word: Option<word::Word> = None;
    let mut depth: u64 = 0;

    macro_rules! push_word {
        () => {
            match &mut word {
                None => {},
                Some(w) => {
                    if w.string.len() != 0 {
                        line.content.push(w.clone());
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
                    word = Some(word::Word {
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
                    if depth == u64::MAX {
                        let mut correct_info = (*info).clone();
                        correct_info.line_number = i;
                        syntax_error(&correct_info, format!("Too much nesting!"))
                    }
                    depth += 1;
                    push_word!();
                }
                else if c == ')' || c == ']' || c == '>' {
                    if depth == 0 {
                        let mut correct_info = (*info).clone();
                        correct_info.line_number = i;
                        syntax_error(&correct_info, format!("Expression contains more closing than opening brackets!"))
                    }

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
            if line.content.len() != 0 {
                line.line_number = i;
                lines.push(line);
                //lines.push(parse_line(line, &file_name, i+1));
                //execute_line(parsed_line, data);
                line = line::Line::new(&file_name, usize::MAX);
            }
        }
    }
    return lines;
}


pub fn run_runk_buffer(input_file_reader: Box<dyn BufRead>, file_name: &str, data: &mut program_data::ProgramData) {
    let mut index = 0;
    let mut info = source_info::SourceInfo::new(index, &file_name, &file_name); // TODO add text
    let lines: Vec<line::Line> = parse_file(input_file_reader, &info, &file_name[..]);

    // looping through every (assignment +) expression.
    while index != lines.len() {
        if data.debug {
            eprint!("{}", format!("RUN {}\t| ", lines[index].line_number).bright_yellow());
            eprint!("{} ", lines[index]);
            eprintln!("");
        }

        info.line_number = index;
        let (assign, exp_start_index) = parse_assignment(&lines[index].content[..], &info);
        let (result, exp_end_index, next_index) = resolve_exp(&lines[index].content[exp_start_index..], &info, &data);

        if exp_start_index + exp_end_index != lines[index].content.len() {
            syntax_error(&info, format!("Unexpected text after expression!"));
        }

        execute_asignment(&assign, &result, &info, data);

        index = match next_index {
            Option::None => index+1,
            Option::Some(i) => i
        };

    }

    if data.debug {
        eprintln!("{}", "DONE".green());
        color_print!("\nVariables:\n", blue italic);
        for (key, val) in data.vars.iter() {
            eprintln!("{}: {}", key, val);
        }
    }
}