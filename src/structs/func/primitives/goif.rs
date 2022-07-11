use super::super::var;
use num_traits::{ Zero };
use super::super::func_return;
use super::super::Func;
use super::super::super::func;

pub fn get_func() -> Func {
    Func {
        func: op,
        args: func::ArgSpec::Limited(
            vec!(
                var::Var::z(num_bigint::ToBigInt::to_bigint(&0).unwrap()).unwrap(),
                var::Var::L(format!("")),
            )
        ),
    }
}

/// Sums all variables.
pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    if let var::Var::Z(cond) = &args[0] {
        if *cond == Zero::zero() {
            return func_return::FuncReturn{
                var: Ok(var::Var::t(format!("")).unwrap()),
                jump_to: None,
            };
        }
    }

    if let var::Var::L(s) = &args[1] {
        return func_return::FuncReturn{
            var: Ok(var::Var::t(format!("")).unwrap()),
            jump_to: Some(s.clone()),
        };
    }

    func_return::FuncReturn::func_error(format!("Could not execute jump"))
}
