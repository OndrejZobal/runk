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
                var::Var::t(format!("")).unwrap(),
            )
        ),
    }
}

/// Sums all variables.
pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    if args.len() < 2 {
        return func_return::FuncReturn{
            var: Err(format!("Too little argumets!")),
            jump_to: None
        };
    }

    for i in 1..args.len() {
        if args[i-1] != args[i] {
            return func_return::FuncReturn{
                var: Ok(var::Var::n(num_bigint::ToBigInt::to_bigint(&0).unwrap()).unwrap()),
                jump_to: None
            };
        }
    }

    return func_return::FuncReturn{
        var: Ok(var::Var::n(num_bigint::ToBigInt::to_bigint(&1).unwrap()).unwrap()),
        jump_to: None
    };

}
