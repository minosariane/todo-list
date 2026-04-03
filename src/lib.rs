pub mod model;
pub use model::error::TaskError;
pub use model::manager::Manager;
pub use model::task;
pub use model::task::Task;
pub mod storage;
pub use storage::Storage;
pub use storage::json_storage::JsonStorage;
pub mod cli;
pub use cli::Cli;
pub use cli::Commands;

#[cfg(test)]
mod tests {

    use super::*;

    mod storage_tests {
        use super::*;
        use std::fs;
        use tempfile::NamedTempFile;

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
        fn json_storage_save_and_load() {
            let file = NamedTempFile::new().unwrap();
            let path = file.path().to_str().unwrap().to_string();

            let storage = JsonStorage { path: path };

            let tasks = vec![Task::new(1, "A".to_string()), Task::new(2, "B".to_string())];

            storage.save(&tasks).unwrap();

            let loaded = storage.load().unwrap();

            assert_eq!(loaded.len(), 2);
            assert_eq!(loaded[0].name, "A");
        }

        #[test]
        fn json_storage_invalid_path() {
            let path = "non-existant.json".to_string();
            let storage = JsonStorage { path: path };
            let result = storage.load();
            assert!(result.is_ok());
            assert!(result.unwrap().is_empty());
        }

        #[test]
        fn json_storage_corrupted_json() {
            let file = NamedTempFile::new().unwrap();
            let path = file.path().to_str().unwrap().to_string();
            fs::write(&path, "Totally not json format").unwrap();

            let storage = JsonStorage { path: path };

            let result = storage.load();
            assert!(result.is_err());
        }
    }

    mod manager_tests {

        use super::*;
        use crate::{storage::memory_storage::MemoryStorage, task::Status};

        #[test]
        fn add_task() {
            let storage = MemoryStorage;
            let mut manager = Manager::new(storage).unwrap();

            manager.add_task("test".to_string()).unwrap();

            assert_eq!(manager.tasks.len(), 1);
            assert_eq!(manager.tasks[0].name, "test");
        }

        #[test]
        fn remove_task() {
            let storage = MemoryStorage;
            let mut manager = Manager::new(storage).unwrap();

            manager.add_task("A".to_string()).unwrap();
            manager.add_task("B".to_string()).unwrap();
            manager.remove_task(1).unwrap();

            assert_eq!(manager.tasks.len(), 1);
            assert_eq!(manager.tasks[0].name, "B");
        }

        #[test]
        fn mark_done() {
            let storage = MemoryStorage;
            let mut manager = Manager::new(storage).unwrap();

            manager.add_task("test".to_string()).unwrap();
            manager.mark_done(1).unwrap();

            assert!(matches!(manager.tasks[0].status, Status::Done));
        }

        #[test]
        fn error_not_found() {
            let storage = MemoryStorage;
            let mut manager = Manager::new(storage).unwrap();
            manager.add_task("test".to_string()).unwrap();

            let remove = manager.remove_task(42);
            assert!(remove.is_err());
            let mark_done = manager.mark_done(42);
            assert!(mark_done.is_err());
        }

        #[test]
        fn full_flow() {
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
}
