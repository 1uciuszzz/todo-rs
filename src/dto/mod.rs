use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateTodoDto {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateTodoDto {
    pub name: String,
    pub is_finish: bool,
}
