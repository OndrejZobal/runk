use super::var;

// Operations
pub mod add;
pub mod sub;
pub mod mul;
pub mod div;
pub mod out;
pub mod debug;

// Structs
pub mod func_return;

/// Struct representing a runk function.
#[derive(Clone)]
pub struct Func {
    // Closure
    pub func: fn(&[var::Var]) -> func_return::FuncReturn,
    // args
    pub args: ArgSpec,
}

/// Enum for specifiing arguments to a function.
#[derive(Clone)]
pub enum ArgSpec {
    /// Function can contain an infinite amout of args. This vector
    /// specifies which types are allowed.
    Unlimited(Vec<var::Var>),
    /// Contains array of vars representing types and their order
    Limited(Vec<var::Var>),
}
