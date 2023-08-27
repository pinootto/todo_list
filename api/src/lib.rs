//! Provides a RESTful web server managing Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a json list of Todos
//! - 'POST /todos': create a new Todo
//! - 'PUT /todos/:id': update a specific Todo
//! - 'DELETE /todos/:id': delete a specific Todo
//!

use ::entity::todo;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use data_service::{
    sea_orm::{Database, DatabaseConnection},
    Mutation as MutationCore, Query as QueryCore,
};
use dotenvy::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

#[tokio::main]
pub async fn main() {
    // let text = match get_from_url("http://45.32.115.191/").await {
    let text = match get_from_url("https://magzdar.net/").await {
        Ok(t) => t,
        Err(err) => format!("WARN: text not found - {err}"),
    };
    println!("text = {text}");

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("database_url = {database_url}");

    let database_conn = Database::connect(database_url)
        .await
        .expect("Database connection failed");

    let state = AppState {
        conn: database_conn,
    };

    let app = Router::new()
        .route("/todos", get(todos_index))
        .route("/todos", post(todos_create))
        .route("/todos/:id", put(todos_update))
        // .route("/todos/:id", delete(todos_delete))
        .with_state(state);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_from_url(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    println!("body = {:?}", body);
    Ok(body)
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

async fn todos_index(State(state): State<AppState>) -> impl IntoResponse {
    let todos = QueryCore::find_all_todos(&state.conn)
        .await
        .expect("Cannot find todos");
    let todo_list = todos
        .iter()
        .map(|todo| Todo {
            id: todo.id,
            text: todo.text.clone(),
            completed: todo.completed,
        })
        .collect::<Vec<_>>();
    Json(todo_list)
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}

async fn todos_create(
    State(state): State<AppState>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    let active_model = MutationCore::create_todo(
        &state.conn,
        todo::Model {
            id: 0,
            text: input.text,
            completed: false,
        },
    )
    .await
    .expect("Cannot create todo");

    // println!("{:#?}", active_model);
    // println!("{:#?}", active_model.id.unwrap());
    // let model: entity::todo::Model = active_model.try_into_model().unwrap();
    //
    let todo = Todo {
        id: active_model.id.unwrap(),
        text: active_model.text.unwrap(),
        completed: active_model.completed.unwrap(),
    };
    (StatusCode::CREATED, Json(todo))
}

#[derive(Debug, Deserialize)]
struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}

async fn todos_update(
    Path(id): Path<i32>,
    State(state): State<AppState>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let found_model = QueryCore::find_todo_by_id(&state.conn, id)
        .await
        .expect(&format!("Cannot find todo by id {}", id));
    let mut found_model = match found_model {
        Some(model) => model,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // let mut todo = db
    //     .read()
    //     .unwrap()
    //     .get(&id)
    //     .cloned()
    //     .ok_or(StatusCode::NOT_FOUND)?;
    if let Some(text) = input.text {
        found_model.text = text;
    }
    if let Some(completed) = input.completed {
        found_model.completed = completed;
    }
    let updated_model = MutationCore::update_todo_by_id(&state.conn, id, found_model)
        .await
        .expect("Cannot update todo");

    Ok(Json(Todo {
        id: updated_model.id,
        text: updated_model.text,
        completed: updated_model.completed,
    }))
}

// async fn todos_delete(Path(id): Path<Uuid>, State(db): State<Db>) -> impl IntoResponse {
//     if db.write().unwrap().remove(&id).is_some() {
//         StatusCode::NO_CONTENT
//     } else {
//         StatusCode::NOT_FOUND
//     }
// }

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
