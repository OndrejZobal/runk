//!
//! Collection of functions dedicated to resolving expressions.
//!

use super::structs::{var, assign, program_data, word, source_info};
use super::structs::func::{ self, func_return };

//use structs::{var::Var, assign::Assign, program_data::ProgramData, word::Word, line::Line};

use crate::prints::*;

// FIXME
fn execute_function(operation: &word::Word,
                    operands: &[var::Var],
                    data: &program_data::ProgramData) -> func_return::FuncReturn {
    match data.funcs.get(&operation.string) {
        Some(f) => {
            match &f.args {
                // Need to ensure that the supplied operands only contains var variants
                // specified in 'vec'
                func::ArgSpec::Unlimited(vec) => {
                    for op in operands {
                        let mut found = false;
                        for supported in vec {
                            if !op.eq_type(supported) {
                                found = true;
                            }
                        }
                        if !found {
                            return func_return::FuncReturn::error(format!(
                                "Unsupported argument type \"{}\" supplied to function \"{}\". (Supported types: {:?})" ,
                                &op, &operation.string, vec));
                        }
                    }
                },
                // Need to ensure the order and variants of supplied operands matches
                // those specified in vec
                func::ArgSpec::Limited(vec) => {
                    // Ensure length of supplied (operands) arguments matches that of reqired args (vec).
                    if operands.len() != vec.len() {
                        return func_return::FuncReturn::error(format!(
                            "Function \"{}\"" ,
                            &operation.string));
                    }
                    for i in 0..operands.len() {
                        if !operands[i].eq_type(&vec[i]) {
                        return func_return::FuncReturn::error(format!(
                            "Function \"{}\"" ,
                            &operation.string));
                        }
                    }
                }
            };
            // TODO check if arguments are ok.
            (f.func)(operands)
        },
        None => func_return::FuncReturn {
            var: Result::Err(format!("Function not found \"{}\"", &operation.string)),
            jump_to: None,
        },
    }

}

fn resolve_function_expression(input: &[word::Word],
                               info: &source_info::SourceInfo,
                               data: &program_data::ProgramData) -> (func_return::FuncReturn, usize) {
    let mut _jump_to: Option<usize> = None;
    let mut operation: Option<word::Word> = None;
    let mut operands: Vec<var::Var> = Vec::new();
    let mut i: usize = 0;

    while i != input.len() {
        if input[i].string == ")" {
            return match operation {
                None => syntax_error(&info, format!("Function name is missing")),
                // Honestly no sure why it's i+2 and not i+1, but if it's not ther the program doesn't skip ) sometimes...
                Some(op) => {
                    let result = execute_function(&op, &operands, &data);
                    (result, i+2)
                }
            }
        }

        // First operand in a function is the operation.
        if i == 0 {
            operation = Some(input[i].clone());
            i += 1;
            continue;
        }

        let (var, end_index) = resolve_exp(&input[i..], &info, &data);
        if var.var.is_err() {
            return (var, end_index);
        }

        operands.push(var.var.unwrap());
        i += end_index;
    }

    syntax_error(&info, format!("Expressions ended abruptly!"));
}

fn resolve_var(input: &[word::Word],
               info: &source_info::SourceInfo,
               data: &program_data::ProgramData) -> func_return::FuncReturn {
    let string = &input[0].string[1..];
    return func_return::FuncReturn {
        var: match data.vars.get(string) {
            None => {
                Err(format!("Variable \"{}\" was not found.", string.italic()))
            },
            Some(num) => {
                Ok((*num).clone())
            },
        },
        jump_to: None,
    };
}


pub fn resolve_exp(input: &[word::Word],
                   info: &source_info::SourceInfo,
                   data: &program_data::ProgramData) ->  (func_return::FuncReturn, usize){
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
        return (resolve_var(&input[..1], &info, data), 1);
    }

    // Simple expression
    return (func_return::FuncReturn {var: var::Var::from_str(input[0].string.as_str()), jump_to: None }, 1) ;
}
