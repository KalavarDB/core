use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum ErrorMap {
    // General Errors
    /// Unknown error, needs to be mapped to error code
    G000,

    /// Error connecting to database
    G101,

    /// Error determining home directory of user due to missing environment variables
    G102,

    /// Failed to generate required files (permissions error)
    G201,

    /// Failed to determine config directory
    G202,

    /// Failed to determine log directory
    G203,

    /// Invalid configuration file
    G204,

    // Connection Errors


    // Query Errors


    // Drive Errors
    /// Unable to parse database file - It may be corrupted
    D501,

    /// Unable to parse database map file - It may be corrupted (This file can be rebuilt, but it will take a lot of time and memory to do so
    D502,

    /// Unable to open a data file (NotFound)
    D503,

    /// Unable to open a data file (PermissionDenied)
    D504,

    /// Unable to open a data file (WouldBlock)
    D505,

    /// Unable to open a data file (TimeOut)
    D506,

    /// Unable to open a data file (Interrupted)
    D507,

    /// Unable to open a data file (UnexpectedEOF)
    D508,

    /// Unable to open a data file (Other (Unknown))
    D509,

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
            ErrorMap::G000 => "G000",
            ErrorMap::G101 => "G101",
            ErrorMap::G102 => "G102",
            ErrorMap::G201 => "G201",
            ErrorMap::G202 => "G202",
            ErrorMap::G203 => "G203",
            ErrorMap::G204 => "G204",
            ErrorMap::D501 => "D501",
            ErrorMap::D502 => "D502",
            ErrorMap::D503 => "D503",
            ErrorMap::D504 => "D504",
            ErrorMap::D505 => "D505",
            ErrorMap::D506 => "D506",
            ErrorMap::D507 => "D507",
            ErrorMap::D508 => "D508",
            ErrorMap::D509 => "D509",
        };

        write!(f, "{}", msg)
    }
}

impl Debug for GeneralError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match self.code {
            ErrorMap::G000 => "G000",
            ErrorMap::G101 => "G101",
            ErrorMap::G102 => "G102",
            ErrorMap::G201 => "G201",
            ErrorMap::G202 => "G202",
            ErrorMap::G203 => "G203",
            ErrorMap::G204 => "G204",
            ErrorMap::D501 => "D501",
            ErrorMap::D502 => "D502",
            ErrorMap::D503 => "D503",
            ErrorMap::D504 => "D504",
            ErrorMap::D505 => "D505",
            ErrorMap::D506 => "D506",
            ErrorMap::D507 => "D507",
            ErrorMap::D508 => "D508",
            ErrorMap::D509 => "D509",
        };

        write!(f, "{}", msg)
    }
}

impl Error for GeneralError {}