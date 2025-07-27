// Standard libraries
// N/A

// Project libraries
// N/A

// External crates
// N/A

#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    Pipe,
    RedirectIn,
    RedirectOut,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\n' => {
                chars.next();
            }
            '|' => {
                tokens.push(Token::Pipe);
                chars.next();
            }
            '<' => {
                tokens.push(Token::RedirectIn);
                chars.next();
            }
            '>' => {
                tokens.push(Token::RedirectOut);
                chars.next();
            }
            _ => {
                let mut word = String::new();
                while let Some(&c) = chars.peek() {
                    if c == ' ' || c == '|' || c == '<' || c == '>' || c == '\n' {
                        break;
                    }
                    word.push(c);
                    chars.next();
                }
                tokens.push(Token::Word(word));
            }
        }
    }

    tokens
}
