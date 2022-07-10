use super::super::var;
use num_traits::{ Zero };
use super::super::func_return;
use super::super::Func;
use super::super::super::func;
use num_bigint::BigInt;

pub fn get_func() -> Func {
    Func {
        func: op,
        args: func::ArgSpec::Limited(
            vec!(
                var::Var::t(format!("")).unwrap(),
            )
        ),
    }
}

pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    if let var::Var::T(t) = args[0].clone() {
        // Oh my god...
        return func_return::FuncReturn{
            var: match var::Var::n(match &t[..].parse::<BigInt>() {
                    // Converting string to bigint
                    Ok(bigint) => bigint.clone(),
                    Err(_e) => return func_return::FuncReturn {
                        var: Err(format!("\"{}\" is not a number!", args[0])),
                        jump_to: None
                    }
                }
            )
            {
                // Can the number fit into this type?
                Ok(v) =>  Ok(v.clone()),
                Err(_e) => Err(format!("Cannot convert \"{}\" to a natural number!", args[0]))
            },
            jump_to: None
        };
    }

    func_return::FuncReturn{
        var: Err(format!("Cannot convert \"{}\" to a number!", args[0])),
        jump_to: None,
    }
}
