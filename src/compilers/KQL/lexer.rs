use std::iter::Peekable;
use std::vec::IntoIter;
use super::language::*;
use crate::compilers::KQL::utilities::{Query, Operation};

/// A type of result which only needs one value for convenience
pub(crate) type Result<T> = std::result::Result<T, String>;

/// An iterator which processes the input text into a set of tokens
pub(crate) struct Lexer {
    /// The raw data of the lexer
    raw_data: Peekable<IntoIter<char>>,
}

impl Lexer {
    /// Convet raw text to a string of tokens
    pub fn from_text(text: &str) -> Self {
        Lexer {
            raw_data: text.chars().collect::<Vec<_>>().into_iter().peekable(),
        }
    }

    fn get_next_char_while(&mut self, raw_token: &mut String, cond: fn(char) -> bool) -> (usize, usize) {
        let (mut ln, mut ch): (usize, usize) = (0, 0);
        loop {
            match self.raw_data.peek() {
                Some(c) if cond(*c) => {
                    ch += 1;
                    ln += 1;
                    raw_token.push(*c);
                    self.raw_data.next();
                }
                _ => {
                    break;
                }
            }
        }

        (ln, ch)
    }

    fn is_identifier(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
}

impl Iterator for Lexer {
    type Item = self::Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ln = 1;
        let mut ch = 1;

        let token: self::Result<Token>;
        let first_char: char;

        loop {
            match self.raw_data.next() {
                Some(c) if c == '\n' => {
                    ch = 1;
                    ln += 1;
                    continue;
                }
                Some(c) if c.is_whitespace() => {
                    ch += 1;
                    continue;
                }
                Some(c) => {
                    ch += 1;
                    first_char = c;
                    break;
                }
                None => return None,
            }
        }


        if Self::is_identifier(first_char) && !first_char.is_numeric() {
            let mut name = first_char.to_string();
            let (l, c) = self.get_next_char_while(&mut name, Self::is_identifier);
            ln += l;
            ch += c;

            if KEYWORDS.contains(&name.to_uppercase().as_str()) {
                token = Ok(Token::Keyword(name));
            } else {
                token = Ok(Token::Identifier(name));
            }
        } else if first_char.is_numeric() {
            let mut value = first_char.to_string();
            let (l, c) = self.get_next_char_while(&mut value, |c| c.is_numeric());
            ln += l;
            ch += c;

            token = match value.parse() {
                Ok(i) => Ok(Token::Literal(Literal::Integer(i))),
                Err(_) => Err(format!("Syntax Error ({}:{}): Integer literal {} is invalid", ln, ch, value)),
            }
        } else if first_char == '"' {
            let mut value = String::new();

            let (l, c) = self.get_next_char_while(&mut value, |c| c != '"');
            ln += l;
            ch += c;

            self.raw_data.next();

            token = Ok(Token::Literal(Literal::Str(value)));
        } else {
            let mut raw = first_char.to_string();
            loop {
                if let Some(peek) = self.raw_data.peek() {
                    raw.push(*peek);
                } else {
                    break;
                }

                if VALID_SYMBOLS.contains(&&raw[..]) {
                    self.raw_data.next();
                } else {
                    raw.pop();
                    break;
                }
            }

            token = match &raw[..] {
                s if s == "//" => {
                    let (l, c) = self.get_next_char_while(&mut String::new(), |c| c != '\n');
                    ln += l;
                    ch += c;

                    self.next()?
                }
                s if VALID_SYMBOLS.contains(&s) => Ok(Token::Symbol(raw)),
                _ => Err(format!("Syntax Error ({}:{}): Unknown token: {}", ln, ch, raw)),
            }
        }

        Some(token)
    }
}
