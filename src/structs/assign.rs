use super::var::Var;

#[derive(Clone)]
pub enum Assign {
    Dec(
        Var, // Type
        String, // Name
    ),
    Nondec(String),
}
