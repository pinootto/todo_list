//! Provides a RESTful web server managing Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a json list of Todos
//! - 'POST /todos': create a new Todo
//! - 'PUT /todos/:id': update a specific Todo
//! - 'DELETE /todos/:id': delete a specific Todo
//!

use axum::{extract::State, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
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
        .with_state(db);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

    // to continue
    // to learn
}

async fn todos_index(State(db): State<Db>) -> impl IntoResponse {
    let todos = db.read().unwrap();
    let todos = todos.values().cloned().collect::<Vec<_>>();
    Json(todos)
}

#[derive(Debug, Serialize, Clone)]
struct Todo {
    id: Uuid,
    text: String,
    completed: bool,
}

type Db = Arc<RwLock<HashMap<Uuid, Todo>>>;
