use crate::{Storage, Task, TaskError, task::Status};

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
            title,
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
