use super::super::var;
use num_traits::{ Zero, One };
use super::super::func_return;
use super::super::Func;
use super::super::super::func;

pub fn get_func() -> Func {
    Func {
        func: op,
        args: func::ArgSpec::Limited(
            vec!(
                var::Var::z(Zero::zero()).unwrap(),
            )
        ),
    }
}

/// Sums all variables.
pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    let temp: num_bigint::BigInt = {
        if args[0] != var::Var::z(Zero::zero()).unwrap() {
            Zero::zero()
        }
        else {
            One::one()
        }
    };

    func_return::FuncReturn{
        var: Ok(var::Var::z(temp).unwrap()),
        jump_to: None
    }
}
