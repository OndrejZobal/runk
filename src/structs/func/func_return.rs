use super::var;
use derive_more::Display;
use crate::structs::word;

pub struct FuncReturn {
    // Either contains the returned variable or, in case of an error, contains
    // an error message and an optional copy of the word struct with the word belonging
    // to the name of the called function (for debug and error reporting).
    pub var: Result<var::Var, (String, Option<word::Word>)>,
    pub jump_to: Option<String>,
}

impl FuncReturn {
    /// Constructor for an error return ment for use in the general codebase.
    pub fn error(string: String, opt_word: Option<word::Word>) -> Self {
        return FuncReturn {
            var: Result::Err((string, opt_word)),
            jump_to: None,
        };
    }

    pub fn func_error(string: String) -> Self {
        return FuncReturn {
            var: Result::Err((string, None)),
            jump_to: None,
        };
    }
}
