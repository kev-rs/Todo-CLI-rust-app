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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Status {
    Done,
    InProgress,
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

    pub fn get_value<'a>(&'a self) -> &'a str{
        &self.value
    }

    pub fn get_status<'a>(&'a self) -> &'a Status {
        &self.status
    }

    pub fn get_created_at<'a>(&'a self) -> &'a str {
        &self.created_at
    }

    pub fn get_updated_at<'a>(&'a self) -> Option<&String> {
        let s = self.updated_at.as_ref();
        s
    }
}


impl Clone for Todo {
    fn clone(&self) -> Self {
        Self {
            value: self.get_value().to_string(),
            created_at: self.get_created_at().to_string(),
            updated_at: self.get_updated_at().as_deref().cloned(),
            ..*self
        }
    }
}