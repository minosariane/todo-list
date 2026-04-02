pub mod json_storage;
pub mod memory_storage;

use crate::model::{error::TaskError, task::Task};

pub trait Storage {
    fn load(&self) -> Result<Vec<Task>, TaskError>;
    fn save(&self, tasks: &Vec<Task>) -> Result<(), TaskError>;
}

