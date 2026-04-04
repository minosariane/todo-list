use std::{fs, io::ErrorKind};

use crate::{Storage, Task, TaskError};

pub struct JsonStorage {
    pub path: String,
}

impl Storage for JsonStorage {
    fn load(&self) -> Result<Vec<Task>, TaskError> {
        match fs::read_to_string(&self.path) {
            Ok(data) => {
                let tasks = serde_json::from_str(&data)
                    .map_err(|e| TaskError::StorageError(e.to_string()))?;
                Ok(tasks)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(vec![]),
            Err(e) => Err(TaskError::StorageError(e.to_string())),
        }
    }

    fn save(&self, tasks: &[Task]) -> Result<(), TaskError> {
        let contents = serde_json::to_string_pretty(&tasks)
            .map_err(|e| TaskError::StorageError(e.to_string()))?;

        fs::write(&self.path, contents).map_err(|e| TaskError::StorageError(e.to_string()))?;

        Ok(())
    }
}
