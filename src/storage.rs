use crate::{Task, TaskError};

pub mod json_storage;
pub mod memory_storage;

pub trait Storage {
    fn load(&self) -> Result<Vec<Task>, TaskError>;

    fn save(&self, _tasks: &Vec<Task>) -> Result<(), TaskError>;
}
