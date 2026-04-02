use crate::{Task, TaskError};

pub mod json_storage;
pub use json_storage::JsonStorage;
pub mod memory_storage;

pub trait Storage {
    fn load(&self) -> Result<Vec<Task>, TaskError>;

    fn save(&self, _tasks: &Vec<Task>) -> Result<(), TaskError>;
}