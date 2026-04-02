pub mod model;
pub use model::Manager;
pub use model::Task;
pub mod storage;
pub use storage::json_storage::JsonStorage;
pub mod cli;
pub use cli::Cli;
pub use cli::Commands;

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::{
        model::{
            error::TaskError,
            manager::Manager,
            task::{Status, Task},
        },
        storage::{Storage, memory_storage::MemoryStorage},
    };

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
            } else {
                Ok(self.tasks.clone())
            }
        }

        fn save(&self, _tasks: &Vec<Task>) -> Result<(), TaskError> {
            if self.save_fail == true {
                Err(TaskError::StorageError("Save fail".into()))
            } else {
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
