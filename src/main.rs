use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get},
};
use sea_orm::{ActiveValue::Set, Database, QueryOrder, entity::prelude::*};

use crate::{
    dto::{CreateTodoDto, UpdateTodoDto},
    entity::todo,
};

mod config;
mod dto;
mod entity;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let config = config::AppConfig::from_env().unwrap();

    let db = Database::connect(&config.database_url).await.unwrap();

    let state = AppState { db };

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(get_all).post(create_todo))
        .route("/todos/{id}", delete(delete_todo).put(update_todo))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.server_address)
        .await
        .unwrap();
    tracing::info!("listening on {}", &config.server_address);

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    String::from("todo app via rust")
}

async fn create_todo(
    state: State<AppState>,
    Json(payload): Json<CreateTodoDto>,
) -> (StatusCode, Json<todo::Model>) {
    let new_todo = todo::ActiveModel {
        name: Set(payload.name),
        ..Default::default()
    };
    let result = new_todo.insert(&state.db).await.unwrap();
    (StatusCode::CREATED, Json(result))
}

async fn get_all(state: State<AppState>) -> (StatusCode, Json<Vec<todo::Model>>) {
    let todo_list = todo::Entity::find()
        .order_by_desc(todo::Column::Id)
        .all(&state.db)
        .await
        .unwrap();
    (StatusCode::OK, Json(todo_list))
}
async fn delete_todo(state: State<AppState>, Path(id): Path<i32>) -> StatusCode {
    let current_todo = todo::Entity::find_by_id(id).one(&state.db).await.unwrap();
    match current_todo {
        Some(todo) => {
            todo.delete(&state.db).await.unwrap();
            StatusCode::OK
        }
        _ => StatusCode::NOT_FOUND,
    }
}

async fn update_todo(
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodoDto>,
) -> (StatusCode, Json<todo::Model>) {
    let todo = todo::Entity::find_by_id(id).one(&state.db).await.unwrap();
    let mut todo: todo::ActiveModel = todo.unwrap().into();
    todo.name = Set(payload.name);
    todo.is_finish = Set(payload.is_finish);
    let todo = todo.update(&state.db).await.unwrap();
    (StatusCode::OK, Json(todo))
}
