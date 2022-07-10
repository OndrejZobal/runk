//!
//! Collection of functions dedicated to resolving expressions.
//!

use crate::structs::{var, program_data, word, source_info};
use num_traits::Zero;
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
            (f.func)(operands)
        },
        None => func_return::FuncReturn {
            var: Result::Err(format!("Function not found \"{}\"", &string)),
            jump_to: None,
        },
    }

}


fn function_fail(input: &[word::Word],
                 info: &source_info::SourceInfo,
                 data: &mut program_data::ProgramData) -> Option<(func_return::FuncReturn, usize)> {
    let mut i: usize = 0;
    let mut found_on_fail_token = false;

    // Find on function fail token.
    while i != input.len() {
        if let rtoken::Rtoken::OnFunctionFail = input[i].rtoken {
            found_on_fail_token = true;
            i += 1;
            break;
        }
        i += 1;
    }

    if !found_on_fail_token {
        return None;
    }

    // Check length.
    if input.len() <= i {
        runtime_error(&info, format!("No lable literal or a function call after function onfail token"));
    }

    match &input[i].rtoken {
        rtoken::Rtoken::FunctionStart => {
            let (func_result, func_i) = resolve_exp(&input[i..], &info, data);
            return Some((func_result, i+func_i));
        },
        rtoken::Rtoken::LableLiteral(l) => {
            return Some((
                func_return::FuncReturn {
                    var: var::Var::n(Zero::zero()),
                    jump_to: Some(l.clone()),
                },
                i+1
            ))
        },
        _ => {
            runtime_error(&info, format!("Error occoured during funcion onfail replacement function."))
        }
    };
}

fn resolve_function_expression(input: &[word::Word],
                               info: &source_info::SourceInfo,
                               data: &mut program_data::ProgramData) -> (func_return::FuncReturn, usize) {
    let mut _jump_to: Option<usize> = None;
    let mut operation: Option<word::Word> = None;
    let mut operands: Vec<var::Var> = Vec::new();
    let mut i: usize = 0;

    while i != input.len() {
        // FIXME Need to look out for "->".
        if let rtoken::Rtoken::FunctionEnd = input[i].rtoken {
            match operation {
                None => syntax_error(&info, format!("Function name is missing")),
                Some(op) => {
                    let mut result = execute_function(&op, &mut operands, &data);
                    // If the current fucntion fails, we try to call a replacement function that
                    // follows the original function after an OnFunctionFail token.
                    if let Result::Err(e) = &result.var {
                        data.set_error(e.clone()); // FIXME this need &mut data
                        let fail = function_fail(&input[i..], &info, data);
                        // Onfail function successfull
                        if fail.is_some() {
                            let (fail_result, fail_i) = fail.unwrap();
                            result = fail_result;
                            i += fail_i;
                        }
                        return (result, i+1);
                    }
                    return (result, i+2);
                }
            }
        }

        // First operand in a function is the operation.
        if i == 0 {
            operation = Some(input[i].clone());
            i += 1;
            continue;
        }

        let (var, end_index) = resolve_exp(&input[i..], &info, data);
        if var.var.is_err() {
            return (var, end_index);
        }

        operands.push(var.var.unwrap());
        i += end_index;
    }

    syntax_error(&info, format!("Expressions ended abruptly!"));
}

fn resolve_var(input: &word::Word,
               data: &mut program_data::ProgramData) -> func_return::FuncReturn {
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
                   data: &mut program_data::ProgramData) ->  (func_return::FuncReturn, usize){
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
    if let rtoken::Rtoken::VariableReference(_string) = &input[0].rtoken {
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
