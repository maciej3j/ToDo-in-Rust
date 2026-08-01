use std::{
    collections::HashMap,
    fmt::{self},
};

use serde::{Deserialize, Serialize};

use crate::enums::TaskStatus;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: TaskStatus,
}

impl fmt::Display for ToDoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            TaskStatus::Pending => {
                write!(f, "Pending: {}", self.title)
            }
            TaskStatus::Done => {
                write!(f, "Done: {}", self.title)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllToDoItems {
    pending: Vec<ToDoItem>,
    done: Vec<ToDoItem>,
}

impl AllToDoItems {
    pub fn from_hashmap(all_items: HashMap<String, ToDoItem>) -> AllToDoItems {
        let mut pending = Vec::new();
        let mut done = Vec::new();
        for (_, item) in all_items {
            match item.status {
                TaskStatus::Pending => {
                    pending.push(item);
                }
                TaskStatus::Done => {
                    done.push(item);
                }
            }
        }
        AllToDoItems { pending, done }
    }
}
