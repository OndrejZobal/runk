//!
//! Collection of functions dedicated to resolving expressions.
//!

use super::structs::{var, assign, program_data, word, source_info};

//use structs::{var::Var, assign::Assign, program_data::ProgramData, word::Word, line::Line};

use crate::prints::*;

/// A function that transforms a word in an array into a variable.
///
/// At this point we are not sure what type this expression
/// needs to be, so we just put it in the one that fits best
/// and if we later down the road need different types we
/// can just convert it.
fn resolve_simple_expression(input: &[word::Word], info: &source_info::SourceInfo) -> var::Var {
    let num = (&input[0].string).parse::<u64>();

    if num.is_ok() {
        return var::Var::N(num.unwrap());
    }

    let num = (&input[0].string).parse::<i64>();
    if num.is_ok() {
        return var::Var::Z(num.unwrap());
    }

    // Failed to parse as simple expression:
    syntax_error(&info, format!("Invalid expression \"{}\"!", &input[0].string.italic())
    );
}

fn resolve_op(word: &word::Word, info: &source_info::SourceInfo) -> Op {
    return match &word.string[..] {
        "+" => {
            Op::Add
        },
        "-" => {
            Op::Sub
        },
        "*" => {
            Op::Mul
        },
        "/" => {
            Op::Div
        },
        _ => {
            syntax_error(&info, format!("Invalid expression \"{}\"!", &word.string.italic()));
        }
    }
}

// FIXME
fn execute_function(operation: &Op, operands: &[var::Var]) -> (var::Var, Option<usize>) {
    let mut sum: i64 = 0;
    for operand in operands {
        match operand {
            var::Var::N(n) => {
                sum += *n as i64;
            }
            var::Var::Z(z) => {
                sum += z;
            }
        }
    }
    (var::Var::Z(sum), None)
}

fn resolve_function_expression(input: &[word::Word], info: &source_info::SourceInfo, data: &program_data::ProgramData) -> (var::Var, usize, Option<usize>) {
    let mut _jump_to: Option<usize> = None;
    let mut operation: Option<Op> = None;
    let mut operands: Vec<var::Var> = Vec::new();
    let mut i: usize = 0;

    while i != input.len() {
        if input[i].string == ")" {
            return match operation {
                None => syntax_error(&info, format!("Function name is missing")),
                // Honestly no sure why it's i+2 and not i+1, but if it's not ther the program doesn't skip ) sometimes...
                Some(op) => {
                    let (result, jump_to) = execute_function(&op, &operands);
                    (result, i+2, jump_to)
                }
            }
        }

        // First operand in a function is the operation.
        if i == 0 {
            operation = Some(resolve_op(&input[i], &info));
            i += 1;
            continue;
        }

        let (num, end_index, _jump_to) = resolve_exp(&input[i..], &info, &data);
        operands.push(num);
        i += end_index;
    }

    syntax_error(&info, format!("Expressions ended abruptly!"));
}

fn resolve_var(input: &[word::Word], info: &source_info::SourceInfo, data: &program_data::ProgramData) -> var::Var {
    let string = &input[0].string[1..];
    return match data.vars.get(string) {
        None => {
            runtime_error(&info, format!("Variable \"{}\" accesed while undefined!", string.italic()));
        }
        Some(num) => {
            (*num).clone()
        }
    };
}


pub fn resolve_exp(input: &[word::Word], info: &source_info::SourceInfo, data: &program_data::ProgramData) -> (var::Var, usize, Option<usize>) {
    let jump_to: Option<usize> = None;

    if input.len() < 1 {
        syntax_error(&info, format!("Missing expression!"));
    }

    // Function expression
    if input[0].string == "(" {
        if input.len() < 2 {
            syntax_error(&info, format!("Expression ends abruptly!"));
        }
        return resolve_function_expression(&input[1..], &info, data);
    }

    // Resolve variable
    if input[0].string.chars().nth(0).unwrap() == '$' {
        return (resolve_var(&input[..1], &info, data), 1, None);
    }

    // Simple expression
    return (resolve_simple_expression(&input[..1], &info), 1, None);
}
