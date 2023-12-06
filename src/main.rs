mod templates;
mod todo_service;

use askama::Template;
use templates::{IndexTemplate, Todo, TodosTemplate};

use axum::{
    extract::State,
    http::StatusCode,
    response::{ErrorResponse, Html, IntoResponse},
    routing::{get, post},
    Form, Router,
};
use todo_service::{TodoService, TodoServiceJSON};

#[derive(Clone)]
pub struct AppState {
    todo_service: TodoServiceJSON,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        todo_service: TodoServiceJSON::new(),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/todos", get(get_todos))
        .route("/add_todo", post(add_todo))
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
    match todo_service.get_todos() {
        Ok(todos) => {
            let template = TodosTemplate { todos };
            let rendered = template.render().unwrap();
            (StatusCode::OK, Html(rendered).into_response())
        }
        Err(err) => {
            println!("Error getting todos: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Internal Server Error").into_response(),
            )
        }
    }
}

async fn add_todo(
    State(AppState { todo_service }): State<AppState>,
    Form(todo): Form<Todo>,
) -> impl IntoResponse {
    if todo.title.is_empty() || todo.description.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Html("Title or description is empty").into_response(),
        );
    }

    match todo_service.add_todo(todo) {
        Ok(_) => (StatusCode::OK, Html("Todo added").into_response()),
        Err(err) => {
            println!("Error getting todos: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Internal Server Error").into_response(),
            )
        }
    }
}
