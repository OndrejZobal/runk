use std::io::{ BufRead };
use std::collections;
use colored::Colorize;
use num_traits::{ Zero };

use crate::structs::{var, assign, program_data, word, source_info, line};
use crate::prints::{syntax_error, runtime_error};
use crate::expressions::resolve_exp;
use crate::color_print;

/// Parses assignment (if it exsits)
fn parse_assignment(input: &[word::Word], info: &source_info::SourceInfo) -> (Option<assign::Assign>, usize) {
    // Parsing assignment
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

    // This assumes that its croppeed

    // Nondeclarative
    if exp_start_index == 1 {
        if input[0].string.parse::<num_bigint::BigInt>().is_ok() {
            syntax_error(&info, format!("Variable name \"{}\" is invalid!", input[0].string.italic()))
        }

        return (
            Some(assign::Assign::Nondec(input[0].string.clone())),
            exp_start_index+1
        );
    }

    // Declarative
    if exp_start_index == 2 {
        if input[1].string.parse::<num_bigint::BigInt>().is_ok() {
            syntax_error(&info, format!("Variable name \"{}\" is invalid!", input[1].string.italic()))
        }
        return (
            Some(assign::Assign::Dec(
            match &input[0].string[..] {
                "Z" => var::Var::z(Zero::zero()).unwrap(),
                "N" => var::Var::n(Zero::zero()).unwrap(),
                "T" => var::Var::t(format!("")).unwrap(),
                "L" => var::Var::l(format!("")).unwrap(),
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
                     value: &var::Var,
                     info: &source_info::SourceInfo,
                     data: &mut program_data::ProgramData) -> Option<usize> {
    // Processing assignment
    if assign.is_none() {
        if data.debug {
            color_print!("Not Assigned: ", yellow);
            eprintln!("{}", value);
        }
        return None;
    }

    let bruh = assign.as_ref().unwrap().clone();
    match bruh {
        assign::Assign::Dec(mut num, ref string) => {
            if data.vars.contains_key(&string[..]) {
                // Runk allows "redefinition" of a variable as long as the type is the same.
                let old = data.vars.get(&string[..]).unwrap();
                if !num.eq_type(&old) {
                    runtime_error(&info, format!("Redefinition of variable \"{}\"!", string.italic()));
                }
            }

            // Copying the acutal number to "num"
            if let Result::Err(e) = value.fit_into(&mut num) {
                runtime_error(&info, e);
            }

            // New variable is beeing defined. We match agains the type specified
            // in 'line'
            data.vars.insert((&string[..]).to_string(), num);
        },
        assign::Assign::Nondec(string) => {
            if !data.vars.contains_key(&string[..]) {
                runtime_error(&info, format!("Variable \"{}\" assigned before definition!", string.italic()));
            }

            let mut old_num: var::Var = data.vars.get(&string[..]).unwrap().clone();

            if value.fit_into(&mut old_num).is_err() {
                runtime_error(&info, format!("Failiure while converting numbers during assignment."));
            }

            // New variable is beeing defined. We match agains the type specified
            // in 'line'
            data.vars.insert((&string[..]).to_string(), old_num);
        },
    }

    None
}


fn is_special_operator(c: &char) -> bool {
    "():".chars().any(|s| s.eq(c))
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
    let mut in_string = false;

    macro_rules! push_word {
        () => {
            match &mut word {
                None => {},
                Some(w) => {
                    if w.string.len() != 0 && !in_string {
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
                else if c == '"' {
                    in_string = !in_string;
                }
                else if c.is_whitespace() && !in_string {
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
                if c == '(' {
                    if depth == u64::MAX {
                        let mut correct_info = (*info).clone();
                        correct_info.line_number = i;
                        syntax_error(&correct_info, format!("Too much nesting!"))
                    }
                    depth += 1;
                    push_word!();
                }
                else if c == ')' {
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


/// Finds all lables in an array of lines and adds them to hash_map.
/// Returns Result<Number of lables found, Error message>
fn load_lables(lines: &[line::Line],
               hash_map: &mut collections::HashMap<String, usize>) -> Result<usize, String> {
    let mut counter = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.content.len() != 1 {
            continue;
        }

        let lable = match get_lable(&line) {
            Some(s) => s,
            None  => continue,
        };

        if hash_map.get(&lable).is_some() {
            return Result::Err(format!("Redefinition of lable \"{}\".", &lable));
        }

        hash_map.insert(lable.clone(), i);
        counter += 1;
    }

    Result::Ok(counter)
}


fn get_lable(line: &line::Line) -> Option<String> {
    if line.content.len() != 1 {
        return None;
    }

    if line.content[0].string.len() <= 1 {
        return None;
    }

    if line.content[0].string.chars().nth(0).unwrap() == '!' {
        return Some((&line.content[0].string[1..]).to_string());
    }

    return None;
}


pub fn run_runk_buffer(input_file_reader: Box<dyn BufRead>, file_name: &str, data: &mut program_data::ProgramData) {
    let mut index = 0;
    let mut info = source_info::SourceInfo::new(index, &file_name, &file_name); // TODO add text
    let lines: Vec<line::Line> = parse_file(input_file_reader, &info, &file_name[..]);

    match load_lables(&lines, &mut data.lables) {
        Ok(n) => {
            if data.debug {
                eprintln!("Found {} lables", n);
            }
        },
        Err(e) => {
            runtime_error(&info, e);
        }
    }

    // looping through every (assignment +) expression.
    while index != lines.len() {
        info.line_number = lines[index].line_number+1;

        // Skipping lables
        if get_lable(&lines[index]).is_some() {
            index += 1;
            continue;
        }

        if data.debug {
            eprint!("{}", format!("RUN {}\t| ", &info.line_number).bright_yellow());
            eprint!("{} ", lines[index]);
            eprintln!("");
        }

        data.add_primitive_functions();

        // Splitting assignment and expression
        let (assign, exp_start_index) = parse_assignment(&lines[index].content[..], &info);
        // Resolves expressions and returns a value;
        let (ret, exp_end_index) = resolve_exp(&lines[index].content[exp_start_index..], &info, &data);

        // Assigns the value from the expression
        match ret.var {
            Ok(v) => {
                if exp_start_index + exp_end_index != lines[index].content.len() {
                    syntax_error(&info, format!("Unexpected text after expression!"));
                }

                execute_asignment(&assign, &v, &info, data);

                // Executes jump
                index = match ret.jump_to {
                    Option::None => index+1,
                    Option::Some(s) => {
                        match data.lables.get(&s) {
                            None => runtime_error(&info, format!("Attempted to jump to an undefined lable \"{}\"", s)),
                            Some(i) => *i,
                        }
                    },
                };
            },
            Err(string) => {
                runtime_error(&info, string);
            }
        }
    }

    if data.debug {
        eprintln!("{}", "DONE".green());
        data.debug_status();
    }
}
