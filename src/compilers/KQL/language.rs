#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Token {
    Identifier(String),
    Literal(Literal),
    Symbol(String),
    Keyword(String),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Literal {
    Integer(i32),
    Str(String),
}

pub(crate) const KEYWORDS: &[&str] = &[
    "GET", "FIELDS", "I-JOIN",
    "O-JOIN", "L-JOIN", "R-JOIN",
    "INSERT"
];

pub(crate) const VALID_SYMBOLS: &[&str] = &[
    "=", "+", "-",
    "*", "/", "==",
    "!=", "<", ">",
    "<=", ">=", "@",
    "->", ":", ";",
    ",", "{", "}",
    "[", "]", "(",
    ")", "#", "/#",
    "#/",
];
