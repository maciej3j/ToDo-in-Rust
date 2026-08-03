//! File: to_do/core/src/enums.rs

use std::fmt;

use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum TaskStatus {
    Done,
    Pending,
}
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Done => {
                write!(f, "Done")
            }
            Self::Pending => {
                write!(f, "Pending")
            }
        }
    }
}

impl TaskStatus {
    pub fn from_string(status: &str) -> Result<TaskStatus, NanoServiceError> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::Done),
            "PENDING" => Ok(TaskStatus::Pending),
            _ => Err(NanoServiceError::new(
                "Invalid status".to_string(),
                NanoServiceErrorStatus::BadRequest,
            )),
        }
    }
}
