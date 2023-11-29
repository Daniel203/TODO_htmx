mod templates;
mod todo_service;

use askama::Template;
use templates::{IndexTemplate, TodosTemplate};

use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
    Router,
};
use todo_service::{ITodoService, TodoService};

#[derive(Clone)]
pub struct AppState {
    todo_service: TodoService,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        todo_service: TodoService::new(),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/todo", post(add_todo))
        .route("/todos", get(get_todos))
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> impl IntoResponse {
    let template = IndexTemplate {};
    let rendered = template.render().unwrap();
    (StatusCode::OK, Html(rendered).into_response())
}

async fn get_todos(State(AppState { todo_service }): State<AppState>) -> impl IntoResponse {
    let template = TodosTemplate {
        todos: todo_service.get_todos(),
    };

    let rendered = template.render().unwrap();
    (StatusCode::OK, Html(rendered).into_response())
}

async fn add_todo(State(AppState { todo_service }): State<AppState>) -> impl IntoResponse {
    unimplemented!()
}
