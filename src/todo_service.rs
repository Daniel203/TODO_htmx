use crate::templates::Todo;

pub trait ITodoService {
    fn new() -> Self;
    fn get_todos(&self) -> Vec<Todo>;
    fn create_todo(&self, todo: Todo) -> Todo;
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
        return self.todos.clone();
    }
    fn create_todo(&self, todo: Todo) -> Todo {
        unimplemented!()
    }
}
