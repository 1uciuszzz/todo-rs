use axum::{
    Router,
    routing::{delete, get},
};
use sea_orm::{Database, entity::prelude::*};

use crate::controller::todo::{
    create_todo_handler, delete_todo_handler, get_all_todo_handler, update_todo_handler,
};

mod config;
mod controller;
mod dto;
mod entity;
mod service;

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
        .route(
            "/todos",
            get(get_all_todo_handler).post(create_todo_handler),
        )
        .route(
            "/todos/{id}",
            delete(delete_todo_handler).put(update_todo_handler),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.server_address)
        .await
        .unwrap();
    tracing::info!("listening on {}", &config.server_address);

    axum::serve(listener, app).await.unwrap();
}
