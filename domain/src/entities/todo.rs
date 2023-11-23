use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ToDoContent {
    pub content: String,
    pub is_completed: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ToDo {
    pub id: i32,
    pub is_completed: bool,
    pub content: String
    
}