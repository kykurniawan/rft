use uuid::Uuid;

use crate::entities::Todo;

pub struct TodoService {
    todos: Vec<Todo>,
}

impl TodoService {
    pub fn new() -> Self {
        TodoService { todos: Vec::new() }
    }

    pub fn find_all(&self, completed: Option<bool>) -> Vec<&Todo> {
        if let Some(complete) = completed {
            self.todos.iter().filter(|t| t.completed == complete).collect()
        } else {
            self.todos.iter().collect()
        }
    }

    pub fn find_by_id(&self, id: Uuid) -> Option<&Todo> {
        self.todos.iter().find(|t| t.id == id)
    }

    pub fn add(&mut self, title: String) -> Result<Uuid, String> {
        let id = Uuid::new_v4();
        self.todos.push(Todo::new(id, title, false));
        Ok(id)
    }

    pub fn edit(&mut self, id: Uuid, title: String) -> Result<(), String> {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.title = title;
            Ok(())
        } else {
            Err(String::from("Todo not found"))
        }
    }

    pub fn delete(&mut self, id: Uuid) -> Result<(), String> {
        if let Some(index) = self.todos.iter().position(|t| t.id == id) {
            self.todos.remove(index);
            Ok(())
        } else {
            Err(String::from("Todo not found"))
        }
    }

    pub fn mark_completed(&mut self, id: Uuid) -> Result<(), String> {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            Ok(())
        } else {
            Err(String::from("Todo not found"))
        }
    }
}
