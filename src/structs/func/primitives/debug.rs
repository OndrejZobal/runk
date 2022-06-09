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
                var::Var::z(Zero::zero()).unwrap()
            )
        ),
    }
}

pub fn op(_args: &[var::Var]) -> func_return::FuncReturn {
    func_return::FuncReturn{
        var: Ok(var::Var::create_numeric_var(None)),
        jump_to: None
    }
}
