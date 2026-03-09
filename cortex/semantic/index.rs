use std::collections::HashMap;

pub struct SemanticIndex {
    pub data: HashMap<String, Vec<f32>>,
}

impl SemanticIndex {
    pub fn new() -> Self {
        Self { data: HashMap::new() }
    }

    pub fn add(&mut self, id: &str, embedding: Vec<f32>) {
        self.data.insert(id.into(), embedding);
    }
}
