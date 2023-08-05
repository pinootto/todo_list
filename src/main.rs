//! Provides a RESTful web server managing Todos.
//!
//! API will be:
//!
//! - `GET /todos`: return a json list of Todos
//! - 'POST /todos': create a new Todo
//! - 'PUT /todos/:id': update a specific Todo
//! - 'DELETE /todos/:id': delete a specific Todo
//!

use std::net::SocketAddr;

use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new();

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
