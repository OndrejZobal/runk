use super::super::var;
use num_traits::{ Zero, One };
use super::super::func_return;
use super::super::Func;
use super::super::super::func;

pub fn get_func() -> Func {
    Func {
        func: op,
        args: func::ArgSpec::Unlimited(
            vec!(
                var::Var::z(Zero::zero()).unwrap(),
            )
        ),
    }
}

/// Sums all variables.
pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    for arg in args {
        if arg != &var::Var::z(Zero::zero()).unwrap() {
            return func_return::FuncReturn{
                var: Ok(var::Var::z(One::one()).unwrap()),
                jump_to: None
            };
        }
    }
    func_return::FuncReturn{
        var: Ok(var::Var::z(Zero::zero()).unwrap()),
        jump_to: None
    }
}
