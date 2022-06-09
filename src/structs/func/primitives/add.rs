use super::super::var;
use num_traits::{ Zero };
use super::super::func_return;
use super::super::Func;
use super::super::super::func;

pub fn get_func() -> Func {
    Func {
        func: op,
        args: func::ArgSpec::Unlimited(
            vec!(
                var::Var::z(Zero::zero()).unwrap(),
                var::Var::n(Zero::zero()).unwrap()
            )
        ),
    }
}

/// Sums all variables.
pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    let mut sum: var::Var = var::Var::z(Zero::zero()).unwrap();
    for arg in args {
        sum += arg.clone();
    }

    func_return::FuncReturn{
        var: Ok(sum.best_fit()),
        jump_to: None
    }
}
