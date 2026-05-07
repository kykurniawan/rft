use uuid::Uuid;

use crate::services::TodoService;

pub struct Application {
    todo_service: TodoService,
}

impl Application {
    pub fn new() -> Self {
        Application {
            todo_service: TodoService::new(),
        }
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
        let completed_todos = self.todo_service.find_all(Some(true));
        let incomplete_todos = self.todo_service.find_all(Some(false));

        println!("Incomplete Todos =====================");
        if incomplete_todos.is_empty() {
            println!("No todos");
        }
        for todo in incomplete_todos {
            todo.print();
            println!("----------------------------------------");
        }

        println!("Completed Todos =====================");
        if completed_todos.is_empty() {
            println!("No todos");
        }
        for todo in completed_todos {
            todo.print();
            println!("----------------------------------------");
        }

        self.print_header();
    }

    pub fn add_todo(&mut self) {
        println!("Enter the title of the todo: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let title = input.trim().to_string();

        match self.todo_service.add(title) {
            Ok(id) => {
                println!("Todo added: {}", id);
            }
            Err(err) => println!("Error: {}", err),
        }

        self.print_header();
    }

    pub fn edit_todo(&mut self) {
        println!("Enter the id of the todo: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let id = match input.trim().parse::<Uuid>() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid id");
                self.edit_todo();
                return;
            }
        };

        if self.todo_service.find_by_id(id).is_none() {
            println!("Todo not found");
            return;
        }

        println!("Enter the new title of the todo: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let title = input.trim().to_string();

        match self.todo_service.edit(id, title) {
            Ok(_) => println!("Todo edited"),
            Err(err) => println!("Error: {}", err),
        }

        self.print_header();
    }

    pub fn delete_todo(&mut self) {
        println!("Enter the id of the todo: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let id = match input.trim().parse::<Uuid>() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid id");
                self.delete_todo();
                return;
            }
        };

        match self.todo_service.delete(id) {
            Ok(_) => println!("Todo deleted"),
            Err(err) => println!("Error: {}", err),
        }

        self.print_header();
    }

    pub fn mark_completed(&mut self) {
        println!("Enter the id of the todo: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let id = match input.trim().parse::<Uuid>() {
            Ok(id) => id,
            Err(_) => {
                println!("Invalid id");
                self.mark_completed();
                return;
            }
        };

        match self.todo_service.mark_completed(id) {
            Ok(_) => println!("Todo marked as completed"),
            Err(err) => println!("Error: {}", err),
        }

        self.print_header();
    }
}
