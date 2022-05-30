/*
 * Collection of functions dedicated to resolving expressions.
*/

#[path = "structs/mod.rs"]
mod structs;
use crate::Word;
use crate::Var;
use crate::Line;
use crate::Assign;
use crate::ProgramData;

//use structs::{var::Var, assign::Assign, program_data::ProgramData, word::Word, line::Line};

#[path = "prints.rs"]
mod prints;
use crate::prints::*;

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn resolve_simple_expression(input: &[Word], file_name: &str, line_number: &u64) -> Var {
    let num = (&input[0].string).parse::<u64>();

    // At this point we are not sure what type this expression
    // needs to be, so we just put it in the one that fits bets
    // and if we later down the road need different types we
    // can just convert it.
    if num.is_ok() {
        return Var::N(num.unwrap());
    }

    let num = (&input[0].string).parse::<i64>();
    if num.is_ok() {
        return Var::Z(num.unwrap());
    }

    // Failed to parse as simple expression:
    syntax_error(file_name, &line_number,
                    format!("Invalid expression \"{}\"!", &input[0].string.italic())
    );
}

fn resolve_op(word: &Word) -> Op {
    Op::Add
}

fn execute_function(operation: &Op, operands: &[Var]) -> Var {
    let mut sum: i64 = 0;
    for operand in operands {
        match operand {
            Var::N(n) => {
                sum += *n as i64;
            }
            Var::Z(z) => {
                sum += z;
            }
        }
    }
    Var::Z(sum)
}

fn resolve_function_expression(input: &[Word], file_name: &str, line_number: &u64, data: &ProgramData) -> (Var, usize) {
    let mut operation: Option<Op> = None;
    let mut operands: Vec<Var> = Vec::new();
    let mut i: usize = 0;
    while i != input.len() {
        if input[i].string == ")" {
            return match operation {
                None => syntax_error(file_name, &line_number, format!("Function name is missing")),
                // Honestly no sure why it's i+2 and not i+1, but if it's not ther the program doesn't skip ) sometimes...
                Some(op) => (execute_function(&op, &operands), i+2),
            }
        }

        // First operand in a function is the operation.
        if i == 0 {
            operation = Some(resolve_op(&input[i]));
            i += 1;
            continue;
        }

        let (num, end_index) = resolve_exp(&input[i..], file_name, line_number, data);
        operands.push(num);
        i += end_index;
    }

    syntax_error(file_name, &line_number, format!("Expressions ended abruptly!"));
}

fn resolve_var(input: &[Word], file_name: &str, line_number: &u64, data: &ProgramData) -> Var {
    let string = &input[0].string[1..];
    return match data.vars.get(string) {
        None => {
            runtime_error(&file_name, &line_number,
                          format!("Variable \"{}\" accesed while undefined!", string.italic())
            );
        }
        Some(num) => {
            (*num).clone()
        }
    };
}

pub fn resolve_exp(input: &[Word], file_name: &str, line_number: &u64, data: &ProgramData) -> (Var, usize) {
    if input.len() < 1 {
        syntax_error(file_name, &line_number, format!("Missing expression!"));
    }


    // Function expression
    if input[0].string == "(" {
        if input.len() < 2 {
            syntax_error(file_name, &line_number, format!("Expression ends abruptly!"));
        }
        return resolve_function_expression(&input[1..], file_name, line_number, data);
    }

    // Resolve variable
    if input[0].string.chars().nth(0).unwrap() == '$' {
        return (resolve_var(&input[..1], file_name, line_number, data), 1);
    }

    // Simple expression
    return (resolve_simple_expression(&input[..1], file_name, line_number), 1);
}
