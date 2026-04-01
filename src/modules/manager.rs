use crate::modules::{
    error::TaskError,
    task::{Status, Task},
};
use super::Storage;

pub struct Manager<S: Storage> {
    storage: S,
    pub tasks: Vec<Task>,
}

impl<S: Storage> Manager<S> {
    pub fn new(storage: S) -> Result<Self, TaskError> {
        let tasks = storage.load()?;
        Ok(Self { storage, tasks })
    }

    pub fn add_task(&mut self, title: String) -> Result<(), TaskError> {
        let task = Task::new(
            self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1,
            title
        );
        self.tasks.push(task);
        self.storage.save(&self.tasks)?;
        Ok(())
    }
    
    pub fn list_task(&self) {
        if !&self.tasks.is_empty() {
            for task in &self.tasks {
                let status = match task.status {
                    Status::Todo => "[ ]",
                    Status::Done => "[x]",
                };
                println!("{} {} - {}", status, task.id, task.name);
            }
        } else {
            println!("No tasks available");
        }
    }

    pub fn mark_done(&mut self, id: usize) -> Result<(), TaskError> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.mark_done();
            self.storage.save(&self.tasks)?;
            Ok(())
        } else {
            Err(TaskError::NotFound(id))
        }
    }

    pub fn remove_task(&mut self, id: usize) -> Result<(), TaskError> {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            self.storage.save(&self.tasks)?;
            Ok(())
        } else {
            Err(TaskError::NotFound(id))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::modules::memory_storage::MemoryStorage;
    use super::*;

    #[derive(Debug)]
    struct MockStorage {
        pub tasks: Vec<Task>,
        pub load_fail: bool,
        pub save_fail: bool,
    }

    impl Storage for MockStorage {
        fn load(&self) -> Result<Vec<Task>, TaskError> {
            if self.load_fail == true {
                Err(TaskError::StorageError("Load fail".into()))
            }else {
                Ok(self.tasks.clone())
            }
        }
    
        fn save(&self, _tasks: &Vec<Task>) -> Result<(), TaskError> {
            if self.save_fail == true {
                Err(TaskError::StorageError("Save fail".into()))
            }else {
                Ok(())
            }
        }
    }

    #[test]
    fn load_fail() {
        let storage = MockStorage {
            tasks: vec![],
            load_fail: true,
            save_fail: false,
        };

        let manager = Manager::new(storage);

        assert!(matches!(manager, Err(TaskError::StorageError(_))));
    }

    #[test]
    fn save_fail() {
        let storage = MockStorage {
            tasks: vec![],
            load_fail: false,
            save_fail: true,
        };

        let mut manager = Manager::new(storage).unwrap();
        let result = manager.add_task("test".to_string());

        assert!(result.is_err());
    }

    #[test]
    fn test_add_task() {
        let storage = MemoryStorage;
        let mut manager = Manager::new(storage).unwrap();

        manager.add_task("test".to_string()).unwrap();

        assert_eq!(manager.tasks.len(), 1);
        assert_eq!(manager.tasks[0].name, "test");
    }

    #[test]
    fn test_remove_task() {
        let storage = MemoryStorage;
        let mut manager = Manager::new(storage).unwrap();

        manager.add_task("task 1".to_string()).unwrap();
        manager.add_task("task 2".to_string()).unwrap();
        manager.remove_task(1).unwrap();

        assert_eq!(manager.tasks.len(), 1);
        assert_eq!(manager.tasks[0].name, "task 2");
    }

    #[test]
    fn test_mark_done() {
        let storage = MemoryStorage;
        let mut manager = Manager::new(storage).unwrap();

        manager.add_task("task 1".to_string()).unwrap();
        manager.mark_done(1).unwrap();

        assert!(matches!(manager.tasks[0].status, Status::Done));
    }

    #[test]
    fn test_error_not_found() {
        let storage = MemoryStorage;
        let mut manager = Manager::new(storage).unwrap();
        manager.add_task("test".to_string()).unwrap();
        let result = manager.remove_task(42);

        assert!(result.is_err());
    }

    #[test]
    fn test_full_flow() {
        let storage = MemoryStorage;
        let mut manager = Manager::new(storage).unwrap();
        manager.add_task("A".to_string()).unwrap();
        manager.add_task("B".to_string()).unwrap();
        manager.mark_done(1).unwrap();
        manager.remove_task(2).unwrap();

        assert_eq!(manager.tasks.len(), 1);
        assert!(matches!(manager.tasks[0].status, Status::Done))
    }
}
