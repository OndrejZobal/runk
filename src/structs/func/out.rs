use super::super::var;

pub fn get_func() -> super::Func {
    super::Func {
        func: op,
        args: super::super::func::ArgSpec::Unlimited(vec!(var::Var::Z(0), var::Var::N(0))),
    }
}

pub fn op(args: &Vec<word::Word>) -> var::Var {
    var::Var::Z(0)
}
