#[cfg(test)]
use crate::{Task, model::TaskError, storage::Storage};

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
