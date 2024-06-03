use std::sync::Mutex;


pub struct IdGenerator {
    id: Mutex<usize>
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator{
            // id 0 is reserved as empty struct
            id: Mutex::new(1)
        }
    }

    pub fn generate_id(&self) -> usize {
        let mut id = self.id.lock().unwrap();
        let r_id = *id;
        *id += 1;
        r_id
    }
}