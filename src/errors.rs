use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum ErrorMap {
    // General Errors
    /// Unknown error, needs to be mapped to error code
    GXXX,

    /// Error connecting to database
    G101,

    /// Failed to generate required files (permissions error)
    G201,

    /// Failed to determine config directory
    G202,

    /// Failed to determine log directory
    G203,

    /// Invalid configuration file
    E204,

    // Connection Errors


    // Query Errors


    // Drive Errors
}

impl Display for ErrorMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A generic error type to handle all the custom errors that this program might throw during operation
struct GeneralError {
    /// The error code which is relevant to the error encapsulated within
    code: ErrorMap,
}

impl Display for GeneralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self.code {
            ErrorMap::GXXX => "GXXX",
            ErrorMap::G101 => "G101",
            ErrorMap::G201 => "G201",
            ErrorMap::G202 => "G202",
            ErrorMap::G203 => "G203",
            ErrorMap::E204 => "G204",
        };

        write!(f, "{}", msg)
    }
}

impl Debug for GeneralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self.code {
            ErrorMap::GXXX => "GXXX",
            ErrorMap::G101 => "G101",
            ErrorMap::G201 => "G201",
            ErrorMap::G202 => "G202",
            ErrorMap::G203 => "G203",
            ErrorMap::E204 => "G204",
        };

        write!(f, "{}", msg)
    }
}

impl Error for GeneralError {}