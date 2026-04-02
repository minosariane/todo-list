#[cfg(test)]
use crate::{model::{Task, TaskError}, storage::Storage};

#[cfg(test)]

#[cfg(test)]
pub struct MemoryStorage;

#[cfg(test)]
impl Storage for MemoryStorage {
    fn load(&self) -> Result<Vec<Task>, TaskError> {
        Ok(vec![])
    }

    fn save(&self, _tasks: &Vec<Task>) -> Result<(), TaskError> {
        Ok(())
    }
}
