use super::super::var;

pub fn get_func() -> super::Func {
    super::Func {
        func: op,
        args: super::super::func::ArgSpec::Unlimited(Vec::<var::Var>::new()),
    }
}

pub fn op(args: &Vec<var::Var>) -> var::Var {
    var::Var::Z(0)
}
