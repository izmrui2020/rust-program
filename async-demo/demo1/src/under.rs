

pub struct Under {
    store: Vec<String>,
}

impl Under {
    pub fn new() -> Self {
        Self {
            store: Vec::new(),
        }
    }

    pub async fn sort(&mut self) {
        self.store.push("hogeg".to_string());
    }
}