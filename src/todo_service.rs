use std::{fs, io::Write};

use crate::templates::Todo;

pub trait TodoService {
    fn new() -> Self;
    fn get_todos(&self) -> Result<Vec<Todo>, anyhow::Error>;
    fn add_todo(&self, todo: Todo) -> Result<(), anyhow::Error> ;
}

#[derive(Clone)]
pub struct TodoServiceJSON {
    pub file_path: String,
}

impl TodoService for TodoServiceJSON {
    fn new() -> Self {
        TodoServiceJSON {
            file_path: String::from("src/todos.json"),
        }
    }

    fn get_todos(&self) -> Result<Vec<Todo>, anyhow::Error> {
        let json_string = fs::read_to_string(&self.file_path)?;
        let todos = serde_json::from_str::<Vec<Todo>>(json_string.as_str())?;
        Ok(todos)
    }

    fn add_todo(&self, todo: Todo) -> Result<(), anyhow::Error> {
        // add the todo to the list of todos
        let mut todos = self.get_todos()?;
        todos.push(todo);
        let updated_json = serde_json::to_vec_pretty(&todos)?;

        // write the data to the file
        let mut file = fs::File::create(&self.file_path)?;
        file.write_all(updated_json.as_slice())?;

        Ok(())
    }
}
