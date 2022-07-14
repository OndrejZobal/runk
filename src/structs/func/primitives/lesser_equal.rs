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
                var::Var::z(Zero::zero()).unwrap(),
                var::Var::z(Zero::zero()).unwrap(),
            )
        ),
    }
}

/// Sums all variables.
pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    if args[0] <= args[1] {
        func_return::FuncReturn{
            var: Ok(var::Var::n(num_bigint::ToBigInt::to_bigint(&1).unwrap()).unwrap()),
            jump_to: None
        }
    }
    else {
        func_return::FuncReturn{
            var: Ok(var::Var::n(num_bigint::ToBigInt::to_bigint(&0).unwrap()).unwrap()),
            jump_to: None
        }
    }
}
