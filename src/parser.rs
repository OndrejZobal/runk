pub mod rtoken;
use std::io::BufRead;
use crate::structs::{ word, source_info, line };
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


struct OpCancel {
    pub string: String,
    pub push_string: PushString,
    pub acc_literally: bool,
}

/// ## Returns
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

pub fn parse_file<'a>(input_file_reader: Box<dyn BufRead>,
                          into: &source_info::SourceInfo,
                          file_name: &'a str) -> Result<Vec::<line::Line<'a>>, (String, usize)> {
    // This variable will eventually be returned.
    let mut lines = Vec::<line::Line>::new();
    // Line of the current interation.
    let mut line = line::Line::new(&file_name, 1);
    // Variable accumulating characters of a word that is currently beeing read.
    let mut accumulator = String::new();
    let mut nesting_stack = Vec::<OpCancel>::new();

    let mut skip_next_c = false;
    let mut acc_literally = false;


    for (i_line, curr_input_line) in input_file_reader.lines().enumerate() {
        // Recording the column where the current word started for runk debug.
        let mut word_start_column = 0;

        macro_rules! push_rtoken {
            ( $acc:ident, $i_line:ident, $i_char:ident ) => {
                if $acc.len() > 0 {
                    match parse_string(&$acc[..]) {
                        Some((rt, cancel)) => {
                            // Pusing word to line
                            line.content.push(
                                word::Word {
                                    rtoken: rt,
                                    original: ($acc),
                                    column: word_start_column,
                                    line: $i_line,
                                    parsed_line: lines.len(),
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

        for i_char in 0..curr_input_line.as_ref().unwrap().chars().count() {

            // For every char in every line
            let c = curr_input_line.as_ref().unwrap().chars().nth(i_char).unwrap();

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
                        push_rtoken!(accumulator, i_line, i_char);
                    }
                    else if let PushString::Separately = nesting_stack.iter().last().unwrap().push_string  {
                        push_rtoken!(accumulator, i_line, i_char);
                        let mut string = c.to_string();
                        push_rtoken!(string, i_line, i_char);

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
                push_rtoken!(accumulator, i_line, i_char);
            }
            // Push acc & then separately c.
            else if is_special_operator(&c) {
                push_rtoken!(accumulator, i_line, i_char);
                let mut string = c.to_string();
                push_rtoken!(string, i_line, i_char);
            }
            // Text literals
            else if c == '"' {
                push_rtoken!(accumulator, i_line, i_char);
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
            push_rtoken!(accumulator, i_line, zero_for_my_beautifull_macro);
        }

        //eprintln!("nesting stack len: {}", nesting_stack.len().to_string());
        if nesting_stack.len() == 0 {
            // Push this line (if line is nt empty)
            if line.content.len() > 0 {
                line.original = curr_input_line.unwrap();
                lines.push(line);
            }
            // Start new line
            // i_line+2 because arrays start with zero (+1) and we are creating struct
            // for the next line (+1).
            line = line::Line::new(&file_name, i_line+2);
        }
    }

    if nesting_stack.len() != 0 {
        return Err((
            format!("Nesting error, missing a closing \"{}\"!",
                    &nesting_stack[nesting_stack.len()-1].string.italic()),
            nesting_stack.len()+1
        ));
    }

    return Ok(lines);
}
