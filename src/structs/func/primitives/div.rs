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
    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            sum = arg.clone();
        }
        else {
            // TODO prevent division by 0
            let zero: num_bigint::BigInt = Zero::zero();
            if arg.clone().to_z().unwrap() == var::Var::Z(zero) {
                return func_return::FuncReturn::func_error(format!("Division by zero!"));
            }
            sum /= arg.clone();
        }
    }

    func_return::FuncReturn{
        var: Ok(sum.best_fit()),
        jump_to: None
    }
}
