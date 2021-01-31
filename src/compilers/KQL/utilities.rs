use crate::core_structures::row::Cell;
use std::fmt::Display;

/// A general structure matching a Query after it has been compiled
#[derive(Debug, Clone)]
pub struct Query {
    /// Any recursive queries which are referenced and must be completed beforehand
    pub recursive: Option<Box<Query>>,

    /// The filter for returned data
    pub filter: Option<Filter>,

    /// The database the query references
    pub database: Option<String>,

    /// The table the query references
    pub table: Option<String>,

    /// Any variables produced by said query
    pub variables: Option<Vec<Variable>>,

    /// The operation the query has requested
    pub operation: Option<Operation>,

    /// The fields the query wishes to return
    pub fields: Option<Vec<String>>,

    /// The values the query is to insert into the table
    pub values: Option<Vec<Cell>>,

}

/// The structure used for filtering data from a query where possible
#[derive(Debug, Clone)]
pub struct Filter {
    pub subs: Vec<SubFilter>,
}

/// A sub-structure used for defining filters on content
#[derive(Debug, Clone)]
pub struct SubFilter {
    /// The (optional) field which the right assignment is being compared against
    pub field: Option<String>,

    /// The (optional) value that the right assignment is being compared against
    pub left: Option<Cell>,

    /// The (required) value that the field or left assignment is being compared against.
    pub right: Option<Cell>,
}

/// A structure defining a variable
#[derive(Debug, Clone)]
pub struct Variable {
    /// The type of variable this is
    pub var_type: VariableType,

    /// The visibility state of the variable
    pub privacy: VariablePrivacy,

    /// The name of the variable
    pub name: String,

    /// The value which it stores
    pub value: Option<Cell>,
}

/// An enumerator defining the types of variables possible
#[derive(Debug, Clone)]
pub enum VariableType {
    /// A type of variable which cannot be changed, making it a constant
    Constant,

    /// A variable which is not constant, meaning it can be be lost at any time (during program shutdown), making it ephemeral
    Ephemeral,
}

/// An enumerator defining the privacy types of variables
#[derive(Debug, Clone)]
pub enum VariablePrivacy {
    /// Can be seen across all connections (Always used for constant values)
    Public,

    /// Can only be seen by the connection which created it
    Private,
}

/// An enumerator defining the different operations allowed to queries
#[derive(Debug, Clone)]
pub enum Operation {
    /// Used on queries which return data
    Get,

    /// Used on queries which provide new data
    Insert,

    /// Used on queries which update existing data
    Modify,

    /// Used on queries which remove existing data
    Prune,
}

/// ErrorMessage from the compiler, not necessarily an error, might be a warning
#[derive(Debug, Clone)]
pub struct CompilerError {
    /// The type of error that has occurred, can also be a warning
    pub e_type: CompilerErrorType,

    /// The formatted error message
    pub formatted: String,

    /// The formatted error message with colors
    pub formatted_color: String,

    /// The headline of the message
    pub headline: String,

    /// The link used in the message for providing additional out-of-bounds support to the user
    pub link: Option<(String, String)>,
}

/// Enum representing the type of error (warning/fatal)
#[derive(Debug, Clone)]
pub enum CompilerErrorType {
    /// For when the error is a warning and the compiler can continue
    Warning,
    Syntax,
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.formatted)
    }
}


impl CompilerError {
    /// Utility function for formatting and generating error and warning messages
    ///
    /// Parameters:
    /// - `message`: `String` - The headline message of the error or warning
    /// - `level`: `usize` - The level of the error (`0` - warning, `1` - error)
    /// - `e_type`: `String` - The type of error (`Warning`, `Syntax Error`)
    /// - `link`: `Option<(String, String)>` - The (optional) link to more information regarding the error/warning
    ///   - `link.0` - The message regarding the link ("For more information see here:")
    ///   - `link.1` - The URL to display`
    pub fn new(headline: String, e_type: CompilerErrorType, link: Option<(String, String)>) -> CompilerError {
        let mut e = CompilerError {
            e_type,
            formatted: "".to_string(),
            formatted_color: "".to_string(),
            headline,
            link,
        };

        e.formatted_color = format_err_message(&e, true);
        e.formatted = format_err_message(&e, false);

        e
    }
}

/// Utility function for formatting and generating error and warning messages
///
/// Parameters:
/// - `e`: &CompilerError - An error instance from which to pull relevant data
/// - `color`: bool - Whether or not the result should contain color escape codes or not
pub fn format_err_message(e: &CompilerError, color: bool) -> String {
    let mut content = String::new();

    let headline = e.headline.clone();
    let lines: Vec<&str> = headline.split("\n").collect();

    let mut index = 0;
    for line in lines {
        if index == 0 {
            match e.e_type {
                CompilerErrorType::Warning => {
                    match color {
                        true => content = format!(" \x1b[1;33mWarning:\x1b[0m \x1b[1m{}\x1b[0m", line),
                        false => content = format!(" Warning: {}", line)
                    }
                }
                CompilerErrorType::Syntax => {
                    match color {
                        true => content = format!(" \x1b[1;31mSyntax Error:\x1b[0m \x1b[1m{}\x1b[0m", line),
                        false => content = format!(" Syntax Error: {}", line)
                    }
                }
                _ => {}
            }
        } else {
            match color {
                true => content = format!("{}\n    \x1b[1;34m|\x1b[0m \x1b[1m{}\x1b[0m", content, line),
                false => content = format!("{}\n    | {}", content, line)
            }
        }

        index += 1;
    }
    if let Some(c) = e.link.clone() {
        match color {
            true => {
                content = format!("{}\n    \x1b[1;34m|\x1b[0m {}\x1b[0m", content, c.0);
                content = format!("{}\n    \x1b[1;34m= {}\x1b[0m", content, c.1);
            }
            false => {
                content = format!("{}\n    | {}", content, c.0);
                content = format!("{}\n    = {}", content, c.1);
            }
        }
    }

    content = format!("{}\n", content);

    content
}