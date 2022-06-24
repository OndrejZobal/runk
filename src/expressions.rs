//!
//! Collection of functions dedicated to resolving expressions.
//!

use crate::structs::{var, program_data, word, source_info};
use crate::structs::func::{ self, func_return };
use crate::parser::rtoken;

//use structs::{var::Var, assign::Assign, program_data::ProgramData, word::Word, line::Line};

use crate::prints::*;


fn execute_function(operation: &word::Word,
                    operands: &mut Vec<var::Var>,
                    data: &program_data::ProgramData) -> func_return::FuncReturn {
    let string;
    if let rtoken::Rtoken::Plain(string_) = &operation.rtoken {
        string = string_.to_string();
    }
    else {
        // Error
        return func_return::FuncReturn::error(format!(
            "Invalid token given as function name \"{}\"." , operation));
    }
    match data.funcs.get(&string) {
        Some(f) => {
            match &f.args {
                // Need to ensure that the supplied operands only contains var variants
                // specified in 'vec'
                func::ArgSpec::Unlimited(vec) => {
                    for op in &mut *operands {
                        let mut found = false;
                        for supported in vec {
                            if !op.eq_type(supported) {
                                found = true;
                            }
                        }
                        if !found {
                            return func_return::FuncReturn::error(format!(
                                "Unsupported argument type \"{}\" supplied to function \"{}\". (Supported types: {:?})" ,
                                &op, &string, vec));
                        }
                    }
                },
                // Need to ensure the order and variants of supplied operands matches
                // those specified in vec
                func::ArgSpec::Limited(vec) => {
                    // Ensure length of supplied (operands) arguments matches that of reqired args (vec).
                    if operands.len() != vec.len() {
                        return func_return::FuncReturn::error(format!(
                            "Call to function \"{}\" doesn't have the required number of argumets." ,
                            &string));
                    }
                    for i in 0..operands.len() {
                        // if !operands[i].eq_type(&vec[i]) {
                        // return func_return::FuncReturn::error(format!(
                        //     "Function \"{}\" called with incorrect argument type." ,
                        //     &operation.string));
                        // }
                        operands[i] = match operands[i].fit_into(&mut vec[i].clone()) {
                            Ok(var) => {
                                var.clone()
                            },
                            Err(e) => {
                                return func_return::FuncReturn {
                                    var: Result::Err(e),
                                    jump_to: None,
                                };
                            },
                        }
                    }
                }
            };
            // TODO check if arguments are ok.
            (f.func)(operands)
        },
        None => func_return::FuncReturn {
            var: Result::Err(format!("Function not found \"{}\"", &string)),
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
        if let rtoken::Rtoken::FunctionEnd = input[i].rtoken {
            return match operation {
                None => syntax_error(&info, format!("Function name is missing")),
                // Honestly no sure why it's i+2 and not i+1, but if it's not ther the program doesn't skip ) sometimes...
                Some(op) => {
                    let result = execute_function(&op, &mut operands, &data);
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

fn resolve_var(input: &word::Word,
               data: &program_data::ProgramData) -> func_return::FuncReturn {
    if let rtoken::Rtoken::VariableReference(string) = &input.rtoken {
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

    func_return::FuncReturn::error(format!("Invalid token \"{}\" for a variable.", input.original.italic()))
}


pub fn resolve_exp(input: &[word::Word],
                   info: &source_info::SourceInfo,
                   data: &program_data::ProgramData) ->  (func_return::FuncReturn, usize){
    if input.len() < 1 {
        syntax_error(&info, format!("Missing expression!"));
    }

    // Function expression
    if let rtoken::Rtoken::FunctionStart = input[0].rtoken {
        if input.len() < 2 {
            syntax_error(&info, format!("Expression ends abruptly!"));
        }
        return resolve_function_expression(&input[1..], &info, data);
    }

    // Resolve variable
    if let rtoken::Rtoken::VariableReference(string) = &input[0].rtoken {
        return (resolve_var(&input[0], data), 1);
    }

    // Resolve literal
    match &input[0].rtoken {
        rtoken::Rtoken::TextLiteral(_string) => return (func_return::FuncReturn {
            var: var::Var::text_from_word(&input[0]),
            jump_to: None,
            }, 1
        ),
        rtoken::Rtoken::LableLiteral(_string) => return (func_return::FuncReturn {
            var: var::Var::lable_from_word(&input[0]),
            jump_to: None,
            }, 1
        ),
        rtoken::Rtoken::NumLiteral(_string) => return (func_return::FuncReturn {
            var: var::Var::num_from_word(&input[0]),
            jump_to: None,
            }, 1
        ),
        _ => {},
    }

    return (func_return::FuncReturn {var: Err(format!("Invalid token \"{}\" in a function.", input[0].original.italic())), jump_to: None }, 1) ;
}
