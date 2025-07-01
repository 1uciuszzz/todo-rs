use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateTodoDto {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoDto {
    pub name: Option<String>,
    pub is_finish: Option<bool>,
}
