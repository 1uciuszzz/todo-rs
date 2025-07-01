use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};

use crate::{
    AppState,
    dto::{
        PaginationParams, PaginationResult,
        todo::{CreateTodoDto, UpdateTodoDto},
    },
    entity::todo,
    service::todo::{create_todo, delete_todo, get_all_todos, update_todo},
};

pub async fn create_todo_handler(
    state: State<AppState>,
    Json(payload): Json<CreateTodoDto>,
) -> (StatusCode, Json<todo::Model>) {
    let new_todo = create_todo(&state.db, payload.name).await;
    (StatusCode::CREATED, Json(new_todo))
}

pub async fn update_todo_handler(
    state: State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTodoDto>,
) -> (StatusCode, Json<todo::Model>) {
    let updated_todo = update_todo(&state.db, id, payload.name, payload.is_finish).await;
    (StatusCode::OK, Json(updated_todo))
}

pub async fn delete_todo_handler(state: State<AppState>, Path(id): Path<i32>) -> StatusCode {
    let is_success = delete_todo(&state.db, id).await;
    if is_success {
        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}

pub async fn get_all_todo_handler(
    state: State<AppState>,
    Query(query): Query<PaginationParams>,
) -> (StatusCode, Json<PaginationResult<Vec<todo::Model>>>) {
    let (todo_list, total) = get_all_todos(&state.db, query.page.unwrap_or(1)).await;
    (
        StatusCode::OK,
        Json(PaginationResult::new(todo_list, total)),
    )
}
