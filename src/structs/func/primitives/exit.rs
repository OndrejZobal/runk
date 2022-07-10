use super::super::var;
use num_traits::{ Zero };
use num_traits::ToPrimitive;
use super::super::func_return;
use super::super::Func;
use super::super::super::func;
use std::process;

pub fn get_func() -> Func {
    Func {
        func: op,
        args: func::ArgSpec::Unlimited(
            vec!(
                var::Var::z(Zero::zero()).unwrap(),
            )
        ),
    }
}

pub fn op(args: &[var::Var]) -> func_return::FuncReturn {
    let mut z = var::Var::z(Zero::zero()).unwrap();
    let _ = args[0].fit_into(&mut z).unwrap();
    if let var::Var::Z(bigint) = z {
        let bbox: Box<dyn ToPrimitive> = Box::new(bigint);
        let code = match bbox.to_i32() {
            Some(i) => i,
            None => return func_return::FuncReturn{
                var: Err(format!("Number is to big to be an exit code!")),
                jump_to: None
            },
        };
        process::exit(code);
    }
    panic!("Internal error in funtion exit");
}
