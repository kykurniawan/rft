use uuid::Uuid;

pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: Uuid, title: String, completed: bool) -> Self {
        Todo {
            id,
            title,
            completed,
        }
    }

    pub fn print(&self) {
        let status = if self.completed {
            String::from("Completed")
        } else {
            String::from("Incomplete")
        };

        println!("[{}] {} - {}", status, self.title, self.id);
    }
}
