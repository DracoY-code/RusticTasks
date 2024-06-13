/// The module that handles the author credentials.
pub mod author;
pub use author::{Author, LoginState};

/// The module that maintains the task details.
pub mod task;
pub use task::{Priority, Status, Task};

use std::{fs, path::Path};

use csv::WriterBuilder;

use crate::RusticError;

/// The to-do list structure to handle the list of
/// tasks that the user will adding to the app.
#[derive(Debug, Default)]
pub struct ToDoList {
    author: Author,
    tasks: Vec<Task>,
}

impl ToDoList {
    /// Initialises the to-do list.
    pub fn build(author: Author) -> Result<Self, RusticError> {
        let tasks: Vec<Task> = Vec::new();
        
        Ok(ToDoList { author, tasks })
    }

    /// Returns an immutable borrow to the `author` field.
    pub fn get_author(&self) -> &Author {
        &self.author
    }

    /// Returns an immutable borrow to the `tasks` field.
    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Adds a new task to the to-do list.
    pub fn add(&mut self, task: Task) {
        self.tasks.push(task);
    }

    /// Writes the to-do list into a CSV file.
    pub fn write_to_csv(&self, file_path: &Path) -> Result<(), RusticError> {
        // Create and open the file if it does not exist
        let file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(file_path)?;
        
        // Check if the file is empty or not
        let is_empty = file.metadata()?.len() == 0;

        // Reopen the file to append new tasks
        let file = fs::OpenOptions::new()
            .append(true)
            .open(file_path)?;
        
        // Create the writer and add headers if the file is empty
        let mut writer = WriterBuilder::new()
            .has_headers(is_empty)
            .from_writer(file);
        
        // Serialise and write the tasks into the file
        for task in &self.tasks {
            writer.serialize(task)?;
        }

        writer.flush()?;

        Ok(())
    }
}
