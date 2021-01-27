use crate::compilers::KQL::lexer::Lexer;
use crate::compilers::KQL::language::Token;
use crate::compilers::KQL::utilities::{Query, Operation};
use super::lexer::Result;

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
                    } else if position == 0 {
                        return Err(format!(r#"Expected one of "GET", "INSERT", "MODIFY", or another operation keyword. Found identifier "{}"
For all operation keywrods, see https://tiny.kalavar.cf/"#, ident));
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