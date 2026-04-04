#[cfg(test)]
use crate::{Storage, Task, TaskError, model};

#[cfg(test)]
pub struct MemoryStorage;

#[cfg(test)]
impl Storage for MemoryStorage {
    fn load(&self) -> Result<Vec<Task>, TaskError> {
        Ok(vec![])
    }

    fn save(&self, _tasks: &[model::task::Task]) -> Result<(), TaskError> {
        Ok(())
    }
}
