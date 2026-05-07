use uuid::Uuid;

use crate::entities::Todo;

pub struct Application {
    todos: Vec<Todo>,
}

impl Application {
    pub fn new() -> Self {
        Application { todos: Vec::new() }
    }

    pub fn print_header(&self) {
        println!("============ Todo Application ============");
        println!("1. List all todos");
        println!("2. Add a todo");
        println!("3. Edit a todo");
        println!("4. Delete a todo");
        println!("5. Mark as completed");
        println!("6. Exit");
        println!("Type your choice: ");
        println!("==========================================");
    }

    pub fn run(&mut self) {
        self.print_header();

        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "1" => self.list_todos(),
                "2" => self.add_todo(),
                "3" => self.edit_todo(),
                "4" => self.delete_todo(),
                "5" => self.mark_completed(),
                "6" => {
                    println!("Exiting...");
                    break;
                }
                _ => println!("Invalid input"),
            }
        }
    }

    pub fn list_todos(&self) {
        if self.todos.is_empty() {
            println!("No todos");
        } else {
            let incomplete_todos = self
                .todos
                .iter()
                .filter(|t| !t.completed)
                .collect::<Vec<&Todo>>();

            let completed_todos = self
                .todos
                .iter()
                .filter(|t| t.completed)
                .collect::<Vec<&Todo>>();

            println!("Incomplete todos:");
            for todo in incomplete_todos {
                todo.print();
            }

            println!("Completed todos:");
            for todo in completed_todos {
                todo.print();
            }
        }

        self.print_header();
    }

    pub fn add_todo(&mut self) {
        println!("Enter the title of the todo: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let title = input.trim().to_string();
        let id = Uuid::new_v4();
        let completed = false;

        self.todos.push(Todo::new(id, title, completed));

        println!("Todo added");

        self.print_header();
    }

    pub fn edit_todo(&mut self) {
        println!("Enter the id of the todo: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let id = input.trim().parse::<Uuid>().unwrap();

        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            println!("Enter the new title of the todo: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            let title = input.trim().to_string();
            todo.title = title;
            todo.print();
        } else {
            println!("Todo not found");
        }

        self.print_header();
    }

    pub fn delete_todo(&mut self) {
        println!("Enter the id of the todo: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let id = input.trim().parse::<Uuid>().unwrap();

        if let Some(index) = self.todos.iter().position(|t| t.id == id) {
            self.todos.remove(index);
            println!("Todo deleted");
        } else {
            println!("Todo not found");
        }

        self.print_header();
    }

    pub fn mark_completed(&mut self) {
        println!("Enter the id of the todo: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let id = input.trim().parse::<Uuid>().unwrap();

        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = true;
            todo.print();
        } else {
            println!("Todo not found");
        }

        self.print_header();
    }
}
