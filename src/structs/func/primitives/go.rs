use super::super::var;
use super::super::func_return;
use super::super::Func;
use super::super::super::func;

pub fn get_func() -> Func {
    Func {
        func: op,
        args: func::ArgSpec::Limited(
            vec!(
                var::Var::L(format!("")),
            )
        ),
    }
}

/// Sums all variables.
pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    if let var::Var::L(s) = &args[0] {
        return func_return::FuncReturn{
            var: Ok(var::Var::T(format!(""))),
            jump_to: Some(s.clone()),
        };
    }

    func_return::FuncReturn::func_error(format!("Could not execute jump"))
}
