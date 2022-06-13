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

pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    for (i, arg) in args.iter().enumerate() {
        print!("{}", arg.plain_string());
        if i != args.len()-1 {
            //print!(" ");
        }
    }
    println!("");

    return func_return::FuncReturn{
        var: Ok(var::Var::create_numeric_var(None)),
        jump_to: None
    }
}
