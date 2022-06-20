//! Runk token
//! Used tokenizing runk source code.

use crate::structs::var;

    /// Runk token
/// Used tokenizing runk source code.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum Rtoken {
    /// Assignment operator. Usually: ":"
    Assign,
    /// One of runk's datatypes. Usually: "N" (for natural number), "Z" (for int)...
    DataType(var::Var),
    /// Token indicating start of function. Usually: "("
    FunctionStart,
    /// Token indicating start of function. Usually: ")"
    FunctionEnd,
    /// Token indicating an alternative action that will be
    /// executed should the function on it's left fail. Usually: "->"
    OnFunctionFail,
    /// A text value boredered by """ on both sides.
    TextLiteral(String),
    /// A value of a (hopefully defined) lable somewhere in the code.
    /// A lable literal has a "!" on the left with no spaces
    LableLiteral(String),
    /// A string that can be converted to a number (with no extra characters.)
    NumLiteral(String),
    /// A refference to variable that is already supposed to exist
    /// string that begginst with a "$"
    VariableReference(String),
    /// Plain text with no other deciferable values.
    Plain(String),
}
