pub mod rtoken;
use std::io::BufRead;
use crate::structs::{ word, line };
use num_bigint::BigInt;
use num_traits::Zero;
use crate::structs::var;
use colored::Colorize;

fn is_special_operator(c: &char) -> bool {
    "(:".chars().any(|s| s.eq(c))
}

enum PushString {
    Yes,
    No,
    Separately,
}


/// # Description
/// Used in the nesting stack. Every instance represents a word that needs to
/// appear in the source in order to pop this instance from the nesting stack.
///
/// For example: opening bracket will create an opcancle with a closing bracket
/// set as the string.
struct OpCancel {
    pub string: String,
    pub push_string: PushString,
    pub acc_literally: bool,
}

/// # Returns
/// Optional tumple with rtoken, and a canceling string for pair operators
/// ex. "(" gets canceled by ")"
fn parse_string(s: &str) -> Option<(rtoken::Rtoken, Option<OpCancel>)> {
    if s.len() < 1 {
        return None;
    }

    // Numeric literals
    if let Ok(_bi) = s.parse::<BigInt>() {
        return Some((rtoken::Rtoken::NumLiteral(s.to_string()), None));
    }

    if s.len() >= 2 {
        // Text literal
        if s.chars().nth(0).unwrap() == '"' && s.chars().last().unwrap() == '"' {
            return Some((rtoken::Rtoken::TextLiteral(s[1..s.len()-1].to_string()), None));
        }

        // Lable literals
        if s.chars().nth(0).unwrap() == '!' {
            return Some((rtoken::Rtoken::LableLiteral(s[1..].to_string()), None));
        }

        // Variable reference
        if s.chars().nth(0).unwrap() == '$' {
            return Some((rtoken::Rtoken::VariableReference(s[1..].to_string()), None));
        }
    }

    if s == ":"  { return Some((rtoken::Rtoken::Assign, None)) }
    if s == "("  { return Some((rtoken::Rtoken::FunctionStart,
                            Some(OpCancel {
                                string: ")".to_string(),
                                push_string: PushString::Separately,
                                acc_literally: false,
                            }))
    )}
    if s == ")"  { return Some((rtoken::Rtoken::FunctionEnd, None)) }
    if s == "->" { return Some((rtoken::Rtoken::OnFunctionFail, None)) }

    // Data types
    // Integer
    if s == "Int" {
        return Some(
            (
                rtoken::Rtoken::DataType(var::Var::z(Zero::zero()).unwrap()),
                None,
            )
    )}
    // Natural number
    if s == "Nat" {
        return Some(
            (
                rtoken::Rtoken::DataType(var::Var::n(Zero::zero()).unwrap()),
                None,
            )
    )}
    // Text
    if s == "Txt" {
        return Some(
            (
                rtoken::Rtoken::DataType(var::Var::t("".to_string()).unwrap()),
                None,
            )
    )}
    // Lable
    if s == "Lab" {
        return Some(
            (
                rtoken::Rtoken::DataType(var::Var::l("".to_string()).unwrap()),
                None,
            )
    )}

    return Some((rtoken::Rtoken::Plain(s.to_string()), None));
}

pub enum ParseResult<'a> {
    /// Line, line number.
    Ok(line::Line<'a>, usize),
    /// File ended nothing read.
    Eof,
    /// Description of the error and a line number on whitch it occoured.
    Err(String, usize),
}

/// # Description
/// Reads runk source code from a buffer and returns a tokenized output.
///
/// # Arguments
/// - `input_file_reader`: Buffer with runk source code
/// - `info`: Information about the sourcefile. TODO Replace with error propagation
/// - `file_name`: name of the file hwere the source comes from. TODO this is already in info.
///
/// # Returns
/// - `Ok`: A vector of (tokenized) Lines.
/// - `Err`:
///     - String: Description of the issue intended to be shown to the user.
///     - usize: Length of the nesting stack. TODO Replace with a line number.
pub fn parse_file<'a>(input_file_reader: &mut Box<dyn BufRead>,
                      file_name: &'a str,
                      line_number: usize,
                      prompt: bool) -> ParseResult<'a> {
    // This variable will eventually be returned.
    // Line of the current interation.
    let mut line             = line::Line::new(&file_name, 1, false);
    // Variable accumulating characters of a word that is currently beeing read.
    let mut accumulator      = String::new();
    let mut nesting_stack    = Vec::<OpCancel>::new();

    let mut skip_next_c      = false;
    let mut acc_literally    = false;
    // Line number relative to the beggining of this effective line.
    let mut curr_line_number = 0;


    loop {
        curr_line_number += 1;
        // Printing a nice prompt for a better REPL experience.
        if prompt {
            if nesting_stack.len() == 0 {
                eprint!("{}", format!("runk) ").green().italic());
            }
            else {
                eprint!("{}", format!("runk [nesting: {}]) ", nesting_stack.len()).green().italic());
            }
        }

        // Read next line.
        let mut curr_input_line = String::new();
        match input_file_reader.read_line(&mut curr_input_line) {
            Ok(code) => {
                // Checking for EOF.
                if code == 0 {
                    if nesting_stack.len() != 0 {
                        return ParseResult::Err(
                            format!("Nesting error, missing a closing \"{}\"!",
                                    &nesting_stack[nesting_stack.len()-1].string.italic()),
                            nesting_stack.len()+1
                        );
                    }
                    // Line will should always be worthless (I think).
                    return ParseResult::Eof;
                }
            },
            Err(e) => return ParseResult::Err(e.to_string(), line_number+curr_line_number),
        };

        // Recording the column where the current word started for runk debug.
        let mut word_start_column = 0;

        // A handy macro for creating words and pushing them.
        macro_rules! push_rtoken {
            ( $acc:ident, $line_number:ident, $i_char:ident ) => {
                if $acc.len() > 0 {
                    match parse_string(&$acc[..]) {
                        Some((rt, cancel)) => {
                            // Pusing word to line
                            line.content.push(
                                word::Word {
                                    rtoken: rt,
                                    original: ($acc),
                                    column: word_start_column,
                                    line: $line_number,
                                    parsed_line: line_number,
                                });
                            // Reseting acc
                            $acc = String::new();
                            word_start_column = $i_char+1;
                            // Push canceling char to nesting stack.
                            match cancel {
                                None => {},
                                Some(s) => {
                                    nesting_stack.push(s);
                                }
                            };
                        },
                        None => {},
                    };
                }
            }
        }


        // For every char in every line
        for i_char in 0..curr_input_line.chars().count() {
            // Current char.
            let c = curr_input_line.chars().nth(i_char).unwrap();

            if skip_next_c == true {
                if acc_literally {
                    accumulator.push(c);
                }

                skip_next_c = false;
                continue;
            }

            // Popping nesting of pair operators like () or ""
            if nesting_stack.len() > 0 {
                if nesting_stack.iter().last().unwrap().string == c.to_string() {
                    if let PushString::Yes = nesting_stack.iter().last().unwrap().push_string {
                        accumulator.push(c);
                        push_rtoken!(accumulator, line_number, i_char);
                    }
                    else if let PushString::Separately = nesting_stack.iter().last().unwrap().push_string  {
                        push_rtoken!(accumulator, line_number, i_char);
                        let mut string = c.to_string();
                        push_rtoken!(string, line_number, i_char);

                    }
                    nesting_stack.pop();

                    if nesting_stack.len() != 0 {
                        acc_literally = nesting_stack.iter().last().unwrap().acc_literally;
                    }
                    else {
                        acc_literally = false;
                    }

                    continue;
                }
            }

            // Skip the following character.
            else if c == '\\' {
                skip_next_c = true;
                continue;
            }
            if acc_literally {
                accumulator.push(c);
                continue;
            }
            // Skip comments.
            if c == '#' {
                break;
            }
            // Push acc, discard c.
            else if c.is_whitespace() {
                push_rtoken!(accumulator, line_number, i_char);
            }
            // Push acc & then separately c.
            else if is_special_operator(&c) {
                push_rtoken!(accumulator, line_number, i_char);
                let mut string = c.to_string();
                push_rtoken!(string, line_number, i_char);
            }
            // Text literals
            else if c == '"' {
                push_rtoken!(accumulator, line_number, i_char);
                acc_literally = true;
                nesting_stack.push(OpCancel {
                    string: "\"".to_string(),
                    push_string: PushString::Yes,
                    acc_literally: true,
                });

                let string = c.to_string();
                accumulator.push_str(&string[..]);
            }
            // c to acc.
            else {
                let string = c.to_string();
                accumulator.push_str(&string[..]);
            }
        }

        if acc_literally {
            // Add the endline to acc because .lines() removes them
            accumulator.push_str("\n");
        }
        else {
            // End of line is also a space, so we need to push acc.
            let zero_for_my_beautifull_macro = 0;
            push_rtoken!(accumulator, line_number, zero_for_my_beautifull_macro);
        }

        if nesting_stack.len() == 0 {
            line.original = curr_input_line[..curr_input_line.len()-1].to_string();
            line.line_number = line_number + curr_line_number;
            if line.content.len() == 0 {
                continue;
            }
            return ParseResult::Ok(line, curr_line_number);
        }
    }
}
