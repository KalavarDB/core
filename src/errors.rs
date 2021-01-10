use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, Copy)]
pub enum ErrorMap {
    // General Errors
    GXXX, // Unknown error, needs to be mapped to error code
    G101, // Error connecting to database
    G201, // Failed to generate required files (permissions error)
    G202, // Failed to determine config directory
    G203, // Failed to determine log directory
    E204, // Invalid configuration file

    // Connection Errors


    // Query Errors


    // Drive Errors

}

impl Display for ErrorMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


struct GeneralError {
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

impl Error for GeneralError {

}