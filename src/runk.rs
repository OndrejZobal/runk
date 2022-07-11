use std::collections;
use colored::Colorize;
use num_traits::{ Zero };
use std::io::{ BufRead, Write };

use crate::structs::{var, assign, program_data, word, source_info, line, optresult};
use crate::prints::fatal_error;
use crate::expressions::resolve_exp;
use crate::parser::{ self, rtoken };
use crate::color_print;

/// Parses assignment (if it exsits)
fn parse_assignment<'a>(input: &'a [word::Word], info: &'a source_info::SourceInfo) -> Result<(Option<assign::Assign>, usize), (String, &'a word::Word)> {
    // Parsing assignment
    let mut exp_start_index: usize = 0;

    // Look for the asssignment operator ":"
    for (i, word) in input.iter().enumerate() {
        if word.rtoken == rtoken::Rtoken::Assign {
            exp_start_index = i;
            break;
        }
    }

    // Check if expression is missing.
    if input.len() <= exp_start_index {
        return Err((format!("Missing expression!"), &input[input.len()-1]));
    }

    // Nondeclarative
    if exp_start_index == 1 {
        if let rtoken::Rtoken::Plain(name) = &input[0].rtoken {
            return Ok((Some(assign::Assign::Nondec(name.to_string())), exp_start_index+1));
        }
        return Err((format!("Variable name \"{}\" is invalid!", input[0].original.italic()), &input[0]));
    }

    // Declarative
    if exp_start_index == 2 {
        if let rtoken::Rtoken::DataType(dtype) = &input[0].rtoken{
            if let rtoken::Rtoken::Plain(name) = &input[1].rtoken {
                return Ok((Some(assign::Assign::Dec(dtype.clone(), name.to_string())), exp_start_index+1));
            }
            return Err((format!("Variable name \"{}\" is invalid!", input[1].original.italic()), &input[1]));
        }
        return Err((format!("Variable type \"{}\" is invalid!", input[0].original.italic()), &input[0]));
    }

    return Ok((None, exp_start_index));
}

/// Executes the tokenized line and assignment
///
/// ## Arguments
/// line The tokenized line
/// data Internal data of the program
fn execute_assignment(assign: &Option<assign::Assign>,
                     value: &var::Var,
                     info: &source_info::SourceInfo,
                     data: &mut program_data::ProgramData) -> Option<usize> {
    // Processing assignment
    if assign.is_none() {
        print!("{}", value.plain_string());
        // Days since caching stdout caused issues: 0
        std::io::stdout().flush().unwrap();
        return None;
    }

    let bruh = assign.as_ref().unwrap().clone();
    match bruh {
        assign::Assign::Dec(mut num, ref string) => {
            if data.vars.contains_key(&string[..]) {
                // Runk allows "redefinition" of a variable as long as the type is the same.
                let old = data.vars.get(&string[..]).unwrap();
                if !num.eq_type(&old) {
                    fatal_error(&info, format!("Redefinition of variable \"{}\"!", string.italic()), None);
                }
            }

            // Copying the acutal number to "num"
            if let Result::Err(e) = value.fit_into(&mut num) {
                fatal_error(&info, e, None);
            }

            // New variable is beeing defined. We match agains the type specified
            // in 'line'
            data.vars.insert((&string[..]).to_string(), num);
        },
        assign::Assign::Nondec(string) => {
            if !data.vars.contains_key(&string[..]) {
                fatal_error(&info, format!("Variable \"{}\" assigned before definition!", string.italic()), None);
            }

            let mut old_num: var::Var = data.vars.get(&string[..]).unwrap().clone();

            if value.fit_into(&mut old_num).is_err() {
                fatal_error(&info, format!("Failiure while converting numbers during assignment."), None);
            }

            // New variable is beeing defined. We match agains the type specified
            // in 'line'
            data.vars.insert((&string[..]).to_string(), old_num);
        },
    }

    None
}

/// Finds all lables in an array of lines and adds them to hash_map.
/// Returns Result<Number of lables found, Error message>
fn load_lables<'a>(lines: &'a [line::Line],
               hash_map: &mut collections::HashMap<String, usize>) -> Result<usize, (String, &'a word::Word)> {
    let mut counter = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.content.len() != 1 {
            continue;
        }

        if let rtoken::Rtoken::LableLiteral(lable) = &line.content[0].rtoken {
            if hash_map.get(&lable[..]).is_some() {
                return Result::Err((format!("Redefinition of lable \"{}\".", &lable), &line.content[0]));
            }

            hash_map.insert(lable.clone(), i);
            counter += 1;
        }
    }

    Result::Ok(counter)
}

pub fn run_runk_buffer(input_file_reader: Box<dyn BufRead>,
                       file_name: &str,
                       data: &mut program_data::ProgramData) {
    let mut index = 0;
    let mut info = source_info::SourceInfo::new(index, &file_name, &file_name); // TODO add text
    let lines: Vec<line::Line> = match parser::parse_file(input_file_reader, &info, &file_name[..]) {
        Err((err, _line_num)) => {
            fatal_error(&info, err, None);
        },
        Ok(l) => l,
    };

    match load_lables(&lines, &mut data.lables) {
        Ok(n) => {
            if data.debug {
                eprintln!("Found {} lables", n);
            }
        },
        Err((s, w)) => {
            info.line_number = w.line;
            info.original = &lines[w.parsed_line].original;
            fatal_error(&info, s, Some(&w));
        }
    }

    // looping through every (assignment +) expression.
    while index != lines.len() {
        info.line_number = lines[index].line_number;
        info.original = &lines[index].original;

        if data.debug {
            eprint!("{}", format!("RUN {}\t| ", &info.line_number).bright_yellow());
            eprint!("{} ", lines[index]);
            eprintln!("");
        }

        data.add_primitive_functions();
        data.add_special_variables();

        // Splitting assignment and expression
        let (assign, exp_start_index) = match parse_assignment(&lines[index].content[..], &info) {
            Ok(tuple) => tuple,
            Err((s, w)) => fatal_error(&info, s, Some(&w)),
        };
        // Resolves expressions and returns a value;
        let (ret, exp_end_index) = resolve_exp(&lines[index].content[exp_start_index..], &info, data);

        // Assigns the value from the expression
        match ret.var {
            Ok(v) => {
                // If line continues beyon what was processed indicates there are incorrect
                // and unneeded tokens following it. (resolve_exp ends when the literal
                // is complete or in case of functions, when the nesting stacks is empty).
                if exp_start_index + exp_end_index < lines[index].content.len() {
                    // Special exception for when the line ends with an unprocessed OnFunctionFail token,
                    // because checking that without processing it would be dirtier than this hack.
                    if lines[index].content[exp_start_index + exp_end_index].rtoken != rtoken::Rtoken::OnFunctionFail {
                        fatal_error(&info,
                                    format!("Unexpected token \"{}\" after expression!", &lines[index].content[exp_start_index+exp_end_index].original),
                                    Some(&lines[index].content[exp_start_index+exp_end_index]),
                        );
                    }
                }

                execute_assignment(&assign, &v, &info, data);

                // Executes jump
                index = match ret.jump_to {
                    Option::None => index+1,
                    Option::Some(s) => {
                        match data.lables.get(&s) {
                            None => fatal_error(&info, format!("Attempted to jump to an undefined lable \"{}\"", s), None),
                            Some(i) => *i,
                        }
                    },
                };
            },
            Err((string, opt_word)) => {
                if opt_word.is_some() {
                    let word = opt_word.unwrap();

                    fatal_error(&info, string, Some(&word));
                }

                fatal_error(&info, string, None);
            }
        }
    }

    if data.debug {
        eprintln!("{}", "DONE".green());
        data.debug_status();
    }
}
