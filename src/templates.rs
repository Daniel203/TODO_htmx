use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {}

#[derive(Template)]
#[template(path = "todos.html")]
pub struct TodosTemplate {
    pub todos: Vec<Todo>,
}

#[derive(Clone)]
pub struct Todo {
    pub title: String,
    pub description: String,
}
