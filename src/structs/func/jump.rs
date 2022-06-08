use super::super::var;
use num_traits::{ Zero };
use super::func_return;

pub fn get_func() -> super::Func {
    super::Func {
        func: op,
        args: super::super::func::ArgSpec::Limited(
            vec!(
                var::Var::L(format!("")),
            )
        ),
    }
}

/// Sums all variables.
pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    if let var::Var::L(s) = &args[0] {
        return func_return::FuncReturn{
            var: Ok(var::Var::z(num_bigint::ToBigInt::to_bigint(&0).unwrap()).unwrap()),
            jump_to: Some(s.clone()),
        };
    }

    func_return::FuncReturn{
        var: Err(format!("Could not execute jump")),
        jump_to: None,
    }
}
