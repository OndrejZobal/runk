//! Runk token
//! Used tokenizing runk source code.

use crate::structs::var;
use derive_more::Display;

/// Runk token
/// Used tokenizing runk source code.
#[derive(Clone, PartialEq, PartialOrd, Debug, Display)]
pub enum Rtoken {
    /// Assignment operator. Usually: ":"
    #[display(fmt = "Assign ")]
    Assign,
    /// One of runk's datatypes. Usually: "N" (for natural number), "Z" (for int)...
    #[display(fmt = "Data Type ")]
    DataType(var::Var),
    /// Token indicating start of function. Usually: "("
    #[display(fmt = "Function Start")]
    FunctionStart,
    /// Token indicating start of function. Usually: ")"
    #[display(fmt = "Function End")]
    FunctionEnd,
    /// Token indicating an alternative action that will be
    /// executed should the function on it's left fail. Usually: "->"
    #[display(fmt = "OnFunction Fail")]
    OnFunctionFail,
    /// A text value boredered by """ on both sides.
    #[display(fmt = "Text Literal")]
    TextLiteral(String),
    /// A value of a (hopefully defined) lable somewhere in the code.
    /// A lable literal has a "!" on the left with no spaces
    #[display(fmt = "LableLiteral")]
    LableLiteral(String),
    /// A string that can be converted to a number (with no extra characters.)
    #[display(fmt = "NumericLiteral")]
    NumLiteral(String),
    /// A refference to variable that is already supposed to exist
    /// string that begginst with a "$"
    #[display(fmt = "VariableReference")]
    VariableReference(String),
    /// Plain text with no other deciferable values.
    #[display(fmt = "Plain")]
    Plain(String),
}
