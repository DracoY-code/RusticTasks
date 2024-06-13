use std::{error, fmt, io};

use thiserror::Error;

/// The main error enumeration that handles error propagation.
#[derive(Debug, Error)]
pub enum RusticError {
    #[error("CSVError encountered!\n\n{0}")]
    CSVError(#[from] csv::Error),

    #[error("DateParserError encountered!\n\n{0}")]
    DateParserError(#[from] chrono::ParseError),

    #[error("HashingError encountered!\n\n{0}")]
    HashingError(#[from] Argon2Error),

    #[error("IOError encountered!\n\n{0}")]
    IOError(#[from] io::Error),

    #[error("JSONError encountered!\n\n{0}")]
    JSONError(#[from] serde_json::Error),

    #[error("RegexError encountered!\n\n{0}")]
    RegexError(#[from] regex::Error),

    #[error("AuthorExists encountered!\n")]
    AuthorExists,

    #[error("InvalidEmail encountered!\n")]
    InvalidEmail,
}

/// The wrapper to handle argon2 errors.
#[derive(Debug)]
pub struct Argon2Error(pub argon2::password_hash::Error);

impl fmt::Display for Argon2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for Argon2Error {}

impl From<argon2::password_hash::Error> for Argon2Error {
    fn from(value: argon2::password_hash::Error) -> Self {
        Argon2Error(value)
    }
}
