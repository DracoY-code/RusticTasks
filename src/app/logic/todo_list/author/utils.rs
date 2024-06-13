use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher,
        SaltString,
    },
    Argon2, PasswordHash, PasswordVerifier,
};
use regex::Regex;

use crate::{Argon2Error, RusticError};

/// Validates the input email address with a simple regex expression.
pub fn is_valid_email(email: &str) -> Result<bool, RusticError> {
    let re = Regex::new(
        r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9,-]+\.[a-zA-Z]{2,}$"
    )?;

    Ok(re.is_match(email))
}

/// Returns an encrypted hash of the password.
pub fn encrypt_password(password: &str) -> Result<String, RusticError> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default().hash_password(
        password.as_bytes(), &salt,
    ).map_err(Argon2Error::from)?;

    Ok(password_hash.to_string())
}

/// Verifies the input password with the saved hash value.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, RusticError> {
    let parsed_hash = PasswordHash::new(&hash).map_err(Argon2Error::from)?;

    Ok(Argon2::default().verify_password(
        password.as_bytes(), &parsed_hash
    ).is_ok())
}
