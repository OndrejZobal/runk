pub mod rtoken;
use std::io::{ BufRead };
use crate::structs::{ word, source_info, line };

fn is_special_operator(c: &char) -> bool {
    "():".chars().any(|s| s.eq(c))
}

struct OpCancel {
    pub string: String,
    pub push_string: bool,
    pub acc_literally: bool,
}

/// ## Returns
/// Optional tumple with rtoken, and a canceling string for pair operators
/// ex. "(" gets canceled by ")"
fn parse_string(s: &str) -> Option<(rtoken::Rtoken, Option<OpCancel>)> {

    if s.len() >= 2 {
        // Text literal
        if s.chars().nth(0).unwrap() == '"'
            && s.chars().nth(s.len()-1).unwrap() == '"' {
                return Some((rtoken::Rtoken::TextLiteral(s[1..s.len()-1].to_string()),
                    Some(OpCancel {
                        string: "\"".to_string(),
                        push_string: false,
                        acc_literally: true,
                    }))
                );
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
                                push_string: true,
                                acc_literally: false,
                            }))
    )}
    if s == ")"  { return Some((rtoken::Rtoken::FunctionEnd, None)) }
    if s == "->" { return Some((rtoken::Rtoken::OnFunctionFail, None)) }

    return Some((rtoken::Rtoken::Plain(s.to_string()), None));
}

pub fn parse_file<'a>(input_file_reader: Box<dyn BufRead>,
                          into: &source_info::SourceInfo,
                          file_name: &'a str) -> Result<Vec::<line::Line<'a>>, String> {
    // This variable will eventually be returned.
    let mut lines = Vec::<line::Line>::new();
    // Line of the current interation.
    let mut line = line::Line::new(&file_name, usize::MAX);
    // Variable for storing currnet word.
    let mut current_string: Option<String> = None;
    // Variable accumulating characters of a word that is currently beeing read.
    let mut accumulator = String::new();
    let mut nesting_stack = Vec::<OpCancel>::new();

    let i_line = 0;
    let i_char = 0;

    macro_rules! push_rtoken {
        ( $acc:ident ) => {
            match parse_string(&$acc[..]) {
                Some((rt, cancel)) => {
                    // Pusing word to line
                    line.content.push(
                        word::Word {
                            string: rt,
                            column: i_char,
                            line: i_line,
                        });
                    // Reseting acc
                    $acc = String::new();
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

    for (i_line, curr_input_line) in input_file_reader.lines().enumerate() {
        let mut skip_next_c = false;
        let mut acc_literally = false;

        for i_char in 0..curr_input_line.unwrap().len() {
            // For every char in every line
            let c = curr_input_line.unwrap().chars().nth(i_char).unwrap();
            if skip_next_c == true {
                skip_next_c = false;
                continue;
            }

            // Popping nesting of pair operators like () or ""
            if nesting_stack.iter().last().unwrap().string == c.to_string() {
                push_rtoken!(accumulator);
                if nesting_stack.iter().last().unwrap().push_string {
                    let string = c.to_string();
                    push_rtoken!(string);
                }
                nesting_stack.pop();

                if nesting_stack.len() != 0 {
                    acc_literally = nesting_stack.iter().last().unwrap().acc_literally;
                }
                else {
                    acc_literally = false;
                }
            }

            if acc_literally {
                accumulator.push(c);
                continue;
            }

            // Skip comments.
            else if c == '#' {
                break;
            }
            // Push acc, discard c.
            else if c.is_whitespace() {
                push_rtoken!(accumulator);
            }
            // Push acc & then separately c.
            else if is_special_operator(&c) {
                push_rtoken!(accumulator);
                let string = c.to_string();
                push_rtoken!(string);
            }
            // Skip the following character.
            else if c == '\\' {
                skip_next_c = true;
            }
            // c to acc.
            else {
                let string = c.to_string();
                accumulator.push_str(&string[..]);
            }
        }

        if nesting_stack.len() == 0 {
            // Push last word
            push_rtoken!(accumulator);
            // Push this line
            lines.push(line);
            // Start new line
            line = line::Line::new(&file_name, usize::MAX);
        }
    }

    return Ok(lines);
}
