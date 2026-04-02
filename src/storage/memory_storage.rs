#[cfg(test)]
use crate::{
    model::{error::TaskError, task::Task},
    storage::Storage,
};

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
