use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, ModelTrait,
    PaginatorTrait, QueryOrder,
};

use crate::entity::todo;

pub async fn create_todo(dbc: &DatabaseConnection, name: String) -> todo::Model {
    let new_todo = todo::ActiveModel {
        name: Set(name),
        ..Default::default()
    };
    let result = new_todo.insert(dbc).await.unwrap();
    result
}

pub async fn get_all_todos(dbc: &DatabaseConnection, page: u64) -> (Vec<todo::Model>, u64) {
    let paginator = todo::Entity::find()
        .order_by_desc(todo::Column::Id)
        .paginate(dbc, 10);
    let todo_list = paginator.fetch_page((page - 1) as u64).await.unwrap();
    let total = paginator.num_items().await.unwrap();
    (todo_list, total)
}

pub async fn delete_todo(dbc: &DatabaseConnection, id: i32) -> bool {
    let target_todo = todo::Entity::find_by_id(id).one(dbc).await.unwrap();
    let result = if let Some(target_todo) = target_todo {
        target_todo.delete(dbc).await.unwrap().rows_affected == 1
    } else {
        false
    };
    result
}

pub async fn update_todo(
    dbc: &DatabaseConnection,
    id: i32,
    name: Option<String>,
    is_finish: Option<bool>,
) -> todo::Model {
    let todo = todo::Entity::find_by_id(id).one(dbc).await.unwrap();
    let mut todo: todo::ActiveModel = todo.unwrap().into();
    if let Some(name) = name {
        todo.name = Set(name);
    };
    if let Some(is_finish) = is_finish {
        todo.is_finish = Set(is_finish);
    };
    let todo = todo.update(dbc).await.unwrap();
    todo
}
