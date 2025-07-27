// Standard libraries
// N/A

// Project libraries
use crate::environment::Environment;
use crate::tokenizer::{Token, tokenize};

// External crates
// N/A

#[derive(Debug)]
pub struct Command {
    pub program: String,
    pub args: Vec<String>,
}

pub fn parse_command(input: &str) -> Option<Command> {
    let tokens: Vec<Token> = tokenize(input);

    let mut iter: std::vec::IntoIter<Token> = tokens.into_iter();
    if let Some(Token::Word(program)) = iter.next() {
        let args: Vec<String> = iter
            .filter_map(|t| {
                if let Token::Word(arg) = t {
                    Some(arg)
                } else {
                    None
                }
            })
            .collect();
        Some(Command { program, args })
    } else {
        None
    }
}

pub fn expand_variables(input: &str, env: &Environment) -> String {
    let mut result: String = String::new();
    let mut chars: std::iter::Peekable<std::str::Chars<'_>> = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            let mut var_name: String = String::new();
            while let Some(&next_char) = chars.peek() {
                if next_char.is_alphanumeric() || next_char == '_' {
                    var_name.push(next_char);
                    chars.next();
                } else {
                    break;
                }
            }
            if let Some(val) = env.get_var(&var_name) {
                result.push_str(val);
            }
        } else {
            result.push(c);
        }
    }

    result
}
