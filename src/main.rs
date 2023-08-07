//! Provides a RESTful web server managing Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a json list of Todos
//! - 'POST /todos': create a new Todo
//! - 'PUT /todos/:id': update a specific Todo
//! - 'DELETE /todos/:id': delete a specific Todo
//!

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let db = Db::default();
    let app = Router::new()
        .route("/todos", get(todos_index))
        .route("/todos", post(todos_create))
        .with_state(db);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // to continue
    // to learn
}

type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: Uuid,
    text: String,
    completed: bool,
}

async fn todos_index(State(db): State<Db>) -> impl IntoResponse {
    let todos = db.read().unwrap();
    let todos = todos.values().cloned().collect::<Vec<_>>();
    Json(todos)
}

#[derive(Debug, Deserialize)]
struct CreateTodo {
    text: String,
}

async fn todos_create(State(db): State<Db>, Json(input): Json<CreateTodo>) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };
    db.write().unwrap().insert(todo.id, todo.clone());
    (StatusCode::CREATED, Json(todo))
}
