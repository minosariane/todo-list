use std::fmt;

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub enum TaskError {
    NotFound(usize),
    StorageError(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::NotFound(id) => write!(f, "Task ID {} not found", id),
            TaskError::StorageError(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}
