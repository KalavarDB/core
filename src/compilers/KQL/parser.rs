use crate::compilers::KQL::lexer::Lexer;
use crate::compilers::KQL::language::Token;
use crate::compilers::KQL::utilities::{Query, Operation};
use super::lexer::Result;
use std::ops::Add;

impl Lexer {
    // Convert the content of the lexer into a Query, or an Error
    pub fn parse(&mut self) -> Result<Query> {
        let mut tree: Vec<Token> = vec![];

        for token_result in self {
            match token_result {
                Ok(token) => {
                    tree.push(token);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        let mut q = Query {
            recursive: None,
            filter: None,
            database: None,
            table: None,
            variables: None,
            operation: None,
            fields: None,
            values: None,
        };

        let mut position = 0;

        for token in tree {
            dbg!(&position, &token);
            match token {
                Token::Identifier(ident) => {
                    if position == 0 {
                        return Err(format_err_message(format!("Expected operation keyword. Found identifier \"{}\"", ident), 1, "Syntax Error".to_string(), Some(("See below for all operation keywords".to_string(), "https://tiny.kalavar.cf/?code=J3kqfgrJ48".to_string()))));
                    } else {
                        match q.operation.clone().unwrap() {
                            Operation::Get => {
                                if position > 0 && position < 4 {
                                    match position {
                                        1 => {
                                            q.database = Some(ident)
                                        }
                                        3 => {
                                            q.table = Some(ident)
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Operation::Insert => {}
                            Operation::Modify => {}
                            Operation::Prune => {}
                        }
                    }
                }
                Token::Literal(lit) => {}
                Token::Symbol(sym) => {}
                Token::Keyword(kw) => {
                    match kw.to_uppercase().as_str() {
                        "GET" => {
                            q.operation = Some(Operation::Get)
                        }
                        "FIELDS" => {}
                        _ => {
                            println!("{}", kw);
                        }
                    }
                }
            }
            position += 1;
        }

        Ok(q)
    }
}

/// Utility function for formatting and generating error and warning messages
///
/// Parameters:
/// - `message`: `String` - The headline message of the error or warning
/// - `level`: `usize` - The level of the error (`0` - warning, `1` - error)
/// - `e_type`: `String` - The type of error (`Warning`, `Syntax Error`)
/// - `link`: `Option<(String, String)>` - The (optional) link to more information regarding the error/warning
///   - `link.0` - The message regarding the link ("For more information see here:")
///   - `link.1` - The URL to display`
pub fn format_err_message(message: String, level: usize, e_type: String, link: Option<(String, String)>) -> String {
    let mut content = String::new();

    let lines: Vec<&str> = message.split("\n").collect();

    let mut index = 0;
    for line in lines {
        if index == 0 {
            match level {
                0 => content = format!(" \x1b[1;33m{}:\x1b[0m \x1b[1m{}\x1b[0m", e_type, line),
                1 => content = format!(" \x1b[1;31m{}:\x1b[0m \x1b[1m{}\x1b[0m", e_type, line),
                _ => {}
            }
        } else {
            content = format!("{}\n    \x1b[1;34m|\x1b[0m \x1b[1m{}\x1b[0m", content, line)
        }

        index += 1;
    }
    if let Some(c) = link {
        content = format!("{}\n    \x1b[1;34m|\x1b[0m {}\x1b[0m", content, c.0);
        content = format!("{}\n    \x1b[1;34m= {}\x1b[0m", content, c.1);
    }

    content = format!("{}\n", content);

    content
}