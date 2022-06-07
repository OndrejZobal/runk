use super::super::var;
use num_traits::{ Zero };
use super::func_return;

pub fn get_func() -> super::Func {
    super::Func {
        func: op,
        args: super::super::func::ArgSpec::Unlimited(
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
