use super::var;

/// Struct representing a runk function.
pub struct Func {
    // Closure
    function: fn(Vec<var::Var>) -> var::Var,
    // args
    args: ArgSpec,
}

/// Enum for specifiing arguments to a function.
pub enum ArgSpec (
    /// Function can contain an infinite amout of args. This vector
    /// specifies which types are allowed.
    Unlimited(Vec<var::Var>),
    /// Contains array of vars representing types and their order
    Limited(Vec<var::Var>),
)
