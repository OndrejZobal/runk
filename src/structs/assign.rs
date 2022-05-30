use crate::Var;

pub enum Assign {
    Dec(
        Var, // Type
        String, // Name
    ),
    Nondec(String),
}
