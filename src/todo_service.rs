use crate::templates::Todo;

pub trait ITodoService {
    fn new() -> Self;
    fn get_todos(&self) -> Vec<Todo>;
    fn add_todo(&mut self, todo: Todo) -> ();
}

#[derive(Clone)]
pub struct TodoService {
    pub todos: Vec<Todo>,
}

impl ITodoService for TodoService {
    fn new() -> Self {
        let todos = vec![
            Todo {
                title: "First todo".to_string(),
                description: "This is the first todo".to_string(),
            },
            Todo {
                title: "Second todo".to_string(),
                description: "This is the second todo".to_string(),
            },
        ];
        Self { todos }
    }
    fn get_todos(&self) -> Vec<Todo> {
        self.todos.clone()
    }
    fn add_todo(&mut self, todo: Todo) -> () {
        self.todos.push(todo.clone());
        ()
    }
}
