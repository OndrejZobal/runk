use super::super::var;
use num_traits::{ Zero };
use super::super::func_return;
use super::super::Func;
use super::super::super::func;
use std::io;

pub fn get_func() -> Func {
    Func {
        func: op,
        args: func::ArgSpec::Unlimited(
            vec!(
                var::Var::z(Zero::zero()).unwrap(),
                var::Var::t(format!("")).unwrap(),
            )
        ),
    }
}

/// Sums all variables.
pub fn op(_args: &[var::Var]) -> func_return::FuncReturn {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    // Remove trailing '\n'
    buffer.pop();
    // Remove trailing '\r' if present
    #[cfg(target_family = "windows")]
    if buffer.chars().last().unwrap() == '\r' {
        buffer.pop();
    }

    return func_return::FuncReturn{
        var: Ok(var::Var::t(buffer).unwrap()), // Low risk unwrap
        jump_to: None
    };
}
