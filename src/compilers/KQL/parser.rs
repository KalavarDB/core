use crate::compilers::KQL::lexer::Lexer;
use crate::compilers::KQL::language::Token;
use crate::compilers::KQL::utilities::{Query, Operation, CompilerError, CompilerErrorType};
// use super::lexer::Result;
use std::ops::Add;

impl Lexer {
    // Convert the content of the lexer into a Query, or an Error
    pub fn parse(&mut self) -> Result<(Query, Vec<CompilerError>), Vec<CompilerError>> {
        let mut tree: Vec<Token> = vec![];
        let mut warnings: Vec<CompilerError> = vec![];
        let mut has_err: bool = false;
        for token_result in self {
            match token_result {
                Ok(token) => {
                    tree.push(token);
                }
                Err(e) => {
                    dbg!(e);
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
            if !has_err {
                dbg!(&position, &token);
                match token {
                    Token::Identifier(ident) => {
                        if position == 0 {
                            has_err = true;
                            warnings.push(CompilerError::new(format!("Expected operation keyword. Found identifier \"{}\"", ident), CompilerErrorType::Syntax, Some(("See below for all operation keywords".to_string(), "https://tiny.kalavar.cf/?code=J3kqfgrJ48".to_string()))))
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
            } else {
                break;
            }
            position += 1;
        }

        return if has_err {
            Err(warnings)
        } else {
            Ok((q, warnings))
        };
    }
}