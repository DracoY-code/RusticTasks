/// The module that contains extra utility
/// functions for the Author structure.
pub mod utils;
use utils::*;

use std::{fs, path::Path};

use chrono::{DateTime, Local};
use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::RusticError;

/// The attributes associated with the current user.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Author {
    id: Uuid,
    name: String,
    email: String,
    password: String,
    created_on: DateTime<Local>,
}

impl Author {
    /// Creates a new Author with the given credentials.
    pub fn build(
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<Self, RusticError> {
        // Validate the email
        let flag = is_valid_email(&email)?;
        if !flag {
            return Err(RusticError::InvalidEmail);
        }

        // Encrypt the password
        let password = encrypt_password(&password)?;

        Ok(Author {
            id: Uuid::new_v4(),
            name: name.to_string(),
            email: email.to_string(),
            password,
            created_on: Local::now(),
        })
    }

    /// Returns an immutable borrow to the `id` field.
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    /// Returns an immutable borrow to the `name` field.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns an immutable borrow to the `email` field.
    pub fn get_email(&self) -> &str {
        &self.email[..]
    }

    /// Returns an immutable borrow to the `password` field.
    pub fn get_password(&self) -> &str {
        &self.password[..]
    }

    /// Returns an immutable borrow to the `created_on` field.
    pub fn get_created_on(&self) -> &DateTime<Local> {
        &self.created_on
    }

    /// Verifies the login credentials and updates the login state.
    pub fn login(
        email: &str,
        password: &str,
        file_path: &Path
    ) -> Result<LoginState, RusticError> {
        // Open the file that contains the login credentials
        let file = fs::OpenOptions::new()
            .read(true)
            .open(file_path)?;

        // Create the reader and add the headers
        let mut reader = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);
        
        // Find the author based on the input email
        for result in reader.deserialize() {
            let author: Author = result?;
            if author.email == email {
                if verify_password(&password, &author.password)? {
                    return Ok(LoginState::LoggedIn);
                } else {
                    return Ok(LoginState::InvalidCredentials);
                }
            }
        }

        Ok(LoginState::DoesNotExist)
    }

    /// Writes the author credentials into a CSV file.
    pub fn write_to_csv(&self, file_path: &Path) -> Result<(), RusticError> {
        // Create and open the file if it does not exist
        let file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;

        // Check if the file is empty or not
        let is_empty = file.metadata()?.len() == 0;

        // Reopen the file in read mode to check for existing authors
        let file = fs::OpenOptions::new()
            .read(true)
            .open(file_path)?;
        
        // Create the reader and add headers if the file is not empty
        let mut reader = ReaderBuilder::new()
            .has_headers(!is_empty)
            .from_reader(file);

        // Deserialise the new author and get the id
        let new_author: Author = serde_json::from_str(&serde_json::to_string(self)?)?;

        // Check if the author already exists
        for result in reader.deserialize() {
            let existing_author: Author = result?;
            if existing_author.email == new_author.email {
                return Err(RusticError::AuthorExists);
            }
        }

        // Reopen the file to append new author
        let file = fs::OpenOptions::new()
            .append(true)
            .open(file_path)?;

        // Create the writer and add headers if the file is empty
        let mut writer = WriterBuilder::new()
            .has_headers(is_empty)
            .from_writer(file);

        // Serialise and write the author into the file
        writer.serialize(self)?;
        writer.flush()?;

        Ok(())
    }
}

/// The current login state.
#[derive(Debug, Default, PartialEq)]
pub enum LoginState {
    LoggedIn,
    DoesNotExist,
    #[default]
    InvalidCredentials,
}
