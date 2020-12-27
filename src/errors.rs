use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum ErrorMap {
    EXXX, // Unknown error, needs to be mapped to error code
    E101, // Error connecting to database
    E201, // Failed to generate required files (permissions error)
    E202, // Failed to determine config directory
    E203, // Failed to determine log directory
    E204, // Invalid configuration file
}

impl Display for ErrorMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}