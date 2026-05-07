use crate::entities::hobby::Hobby;

pub struct User {
    id: u32,
    name: String,
    hobbies: Vec<Hobby>,
}

impl User {
    pub fn new(id: u32, name: String, hobbies: Vec<Hobby>) -> Self {
        User { id, name, hobbies }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_hobbies(&self) -> &[Hobby] {
        &self.hobbies
    }
}
