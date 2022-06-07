use super::super::var;
use num_traits::{ Zero };
use super::func_return;

pub fn get_func() -> super::Func {
    super::Func {
        func: op,
        args: super::super::func::ArgSpec::Unlimited(
            vec!(
                var::Var::z(Zero::zero()).unwrap(),
                var::Var::z(Zero::zero()).unwrap()
            )
        ),
    }
}

pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    func_return::FuncReturn{
        var: Ok(var::Var::create_numeric_var(None)),
        jump_to: None
    }
}
