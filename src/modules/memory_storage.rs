#[cfg(test)]
use crate::modules::{error::TaskError, task::Task};

#[cfg(test)]
pub struct MemoryStorage;

#[cfg(test)]
impl super::Storage for MemoryStorage {
    fn load(&self) -> Result<Vec<super::task::Task>, TaskError> {
        Ok(vec![])
    }
    
    fn save(&self, _tasks: &Vec<Task>) -> Result<(), TaskError> {
        Ok(())
    }
}
