use crate::modules::{error::TaskError, task::Task};

pub mod json_storage;
pub mod memory_storage;
pub mod task;
pub mod manager;
pub mod error;

pub(crate) trait Storage {
    fn load(&self) -> Result<Vec<Task>, TaskError>;
    fn save(&self, tasks: &Vec<Task>) -> Result<(), TaskError>;
}
