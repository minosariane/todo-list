use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Status {
    Todo,
    Done,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub status: Status,
}

impl Task {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            status: Status::Todo,
        }
    }

    pub fn mark_done(&mut self) {
        self.status = Status::Done;
    }
}
