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
                var::Var::n(Zero::zero()).unwrap(),
                var::Var::t(format!("")).unwrap(),
            )
        ),
    }
}

pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    let mut string = String::new();
    for arg in args.iter() {
        string.push_str(&arg.plain_string());
    }

    string += "\n";
    return func_return::FuncReturn{
        var: Ok(var::Var::t(string).unwrap()),
        jump_to: None
    }
}
