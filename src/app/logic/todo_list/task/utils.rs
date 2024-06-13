use chrono::{DateTime, Local};
use serde::Serializer;

use crate::error::RusticError;

/// Serialises the labels field into a comma-separated string.
pub fn serialize_labels<S>(
    labels: &Vec<String>,
    serializer: S
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let labels_str = labels.join(",");
    serializer.serialize_str(&labels_str)
}

/// Parses the due date from the input string.
pub fn parse_due_date(date_str: &str) -> Result<DateTime<Local>, RusticError> {
    let due_date: DateTime<Local> = DateTime::parse_from_str(
        date_str,
        "%Y-%m-%dT%H:%M:%S%z"
    ).map(|dt| dt.with_timezone(&Local))?;
    Ok(due_date)
}
