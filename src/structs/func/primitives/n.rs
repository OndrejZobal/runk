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
                var::Var::t(format!("")).unwrap(),
            )
        ),
    }
}

/// Sums all variables.
pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
        func_return::FuncReturn{
            var: Ok(var::Var::n(num_bigint::ToBigInt::to_bigint(&0).unwrap()).unwrap()),
            jump_to: None
        }

}
