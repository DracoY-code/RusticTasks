/// The module that contains extra utility
/// functions for the Task structure.
pub mod utils;
use utils::*;

use chrono::{DateTime, Local};
use serde::Serialize;
use uuid::Uuid;

use super::{Author, RusticError};

/// The attributes associated with the task.
#[derive(Debug, Default, Serialize)]
pub struct Task {
    id: Uuid,
    name: String,
    description: Option<String>,
    created_on: DateTime<Local>,
    due_date: Option<DateTime<Local>>,
    priority: Priority,
    status: Status,
    #[serde(serialize_with = "serialize_labels")]
    labels: Vec<String>,
    completed_on: Option<DateTime<Local>>,
    created_by: String,
}

impl Task {
    /// Creates a new Task.
    pub fn new(
        name: &str,
        description: Option<&str>,
        due_date: Option<&str>,
        priority: u32,
        labels: Vec<&str>,
        author: &Author,
    ) -> Result<Self, RusticError> {
        Ok(Task {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.map(|desc| desc.to_string()),
            created_on: Local::now(),
            due_date: due_date.map(|dt| parse_due_date(&dt)).transpose()?,
            priority: Priority::set(priority),
            status: Status::Pending,
            labels: labels.iter().map(|&s| s.to_string()).collect(),
            completed_on: None,
            created_by: author.get_name().to_string(),
        })
    }

    /// Returns an immutable borrow to the `id` field.
    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    /// Returns an immutable borrow to the `name` field.
    pub fn get_name(&self) -> &str {
        &self.name[..]
    }

    /// Returns an immutable borrow to the `description` field.
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_ref().map(|desc| &desc[..])
    }

    /// Returns an immutable borrow to the `created_on` field.
    pub fn get_created_on(&self) -> &DateTime<Local> {
        &self.created_on
    }

    /// Returns an immutable borrow to the `due_date` field.
    pub fn get_due_date(&self) -> Option<&DateTime<Local>> {
        self.due_date.as_ref()
    }

    /// Returns an immutable borrow to the `priority` field.
    pub fn get_priority(&self) -> &Priority {
        &self.priority
    }

    /// Returns an immutable borrow to the `status` field.
    pub fn get_status(&self) -> &Status {
        &self.status
    }

    /// Returns an immutable borrow to the `labels` field.
    pub fn get_labels(&self) -> &Vec<String> {
        &self.labels
    }

    /// Returns an immutable borrow to the `completed_on` field.
    pub fn get_completed_on(&self) -> Option<&DateTime<Local>> {
        self.completed_on.as_ref()
    }

    /// Returns an immutable borrow to the `created_by` field.
    pub fn get_created_by(&self) -> &str {
        &self.created_by[..]
    }

    /// Updates the description of the task.
    pub fn update_description(&mut self, desc: &str) {
        self.description = Some(desc.to_string());
    }

    /// Changes the priority level of the task: \[
    ///     Low = 1,
    ///     Medium = 2,
    ///     High = 3
    /// \]
    pub fn change_priority(&mut self, priority: u32) {
        self.priority = Priority::set(priority);
    }

    /// Updates the status of the task.
    pub fn update_status(&mut self, status: Status) {
        self.status = status;
    }

    /// Updates the labels associated with the task.
    pub fn update_labels(&mut self, labels: Vec<&str>) {
        self.labels = labels.iter().map(|&s| s.to_string()).collect();
    }

    /// Sets the `completed_on` field of the task to the current time.
    pub fn mark_completed(&mut self) {
        self.completed_on = Some(Local::now());
    }
}

/// The task priority level.
#[derive(Debug, Default, Serialize)]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
}

impl Priority {
    /// Returns the priority based on the value: \[
    ///     Low = 1,
    ///     Medium = 2,
    ///     High = 3
    /// \]
    pub fn set(value: u32) -> Self {
        return match value {
            1 => Priority::Low,
            2 => Priority::Medium,
            3 => Priority::High,
            _ => Priority::Medium,
        };
    }
}

/// The task completion status.
#[derive(Debug, Default, Serialize)]
pub enum Status {
    #[default]
    Pending,
    InProgress,
    Complete,
}
