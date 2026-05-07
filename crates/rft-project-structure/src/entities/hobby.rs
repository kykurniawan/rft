pub struct Hobby {
    id: u32,
    name: String,
}

impl Hobby {
    pub fn new(id: u32, name: String) -> Self {
        Hobby { id, name }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}
