use super::var;

pub struct FuncReturn {
    pub var: Result<var::Var, String>,
    pub jump_to: Option<String>,
}

impl FuncReturn {
    pub fn error(string: String) -> Self {
        return FuncReturn {
            var: Result::Err(string),
            jump_to: None,
        };
    }
}
