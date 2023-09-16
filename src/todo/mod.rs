use uuid::Uuid as ID;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "todoId")]
    todo_id: ID,
    value: String,
    created_at: String,
    updated_at: Option<String>,
    status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Done,
    IN_PROGRESS,
    IDLE
}

impl Todo {
    pub fn new(todo: String) -> Todo {
        let local: DateTime<Local> = Local::now();
        
        Todo {
            todo_id: ID::new_v4(),
            value: todo,
            created_at: local.to_rfc2822(),
            updated_at: None,
            status: Status::IDLE
        }
    }
}