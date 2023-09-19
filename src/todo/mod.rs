use uuid::Uuid as ID;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    #[serde(rename = "todoId")]
    pub todo_id: ID,
    value: String,
    created_at: String,
    updated_at: Option<String>,
    status: Status,
    priority: Priority,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Status {
    Done,
    InProgress,
    IDLE
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Priority {
    High,
    Medium,
    Low,
    None
}

impl Todo {
    pub fn new(todo: String) -> Todo {
        let local: DateTime<Local> = Local::now();
        
        Todo {
            todo_id: ID::new_v4(),
            value: todo,
            created_at: local.to_rfc2822(),
            updated_at: None,
            status: Status::IDLE,
            priority: Priority::None
        }
    }

    pub fn get_value<'a>(&'a self) -> &'a str{
        &self.value
    }

    pub fn set_value(&mut self, value: String) {
        self.value.clear();
        self.value = value;
    }

    pub fn get_status<'a>(&'a self) -> &'a Status {
        &self.status
    }

    pub fn set_status(&mut self, status: &str) {
        match status.to_lowercase().as_ref() {
            "completed" | "done" | "finished" => self.status = Status::Done,
            "in progress" | "not finished" => self.status = Status::InProgress,
            _ => self.status = Status::IDLE
        }
    }

    pub fn get_priority<'a>(&'a self) -> &'a Priority {
        &self.priority
    }

    pub fn set_priority(&mut self, priority: &str) {
        match priority.to_lowercase().as_ref() {
            "high" | "important" => self.priority = Priority::High,
            "medium" => self.priority = Priority::Medium,
            _ => self.priority = Priority::Low
        }
    }

    pub fn get_created_at<'a>(&'a self) -> &'a str {
        &self.created_at
    }

    pub fn get_updated_at<'a>(&'a self) -> Option<&String> {
        let s = self.updated_at.as_ref();
        s
    }

    pub fn set_updated_at(&mut self) {
        self.updated_at = Some(Local::now().to_rfc2822());
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