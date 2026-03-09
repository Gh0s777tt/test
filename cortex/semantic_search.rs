use std::collections::HashMap;

pub struct SemanticIndex {
    pub index: HashMap<String, Vec<String>>,
}

impl SemanticIndex {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, id: &str, text: &str) {
        let tokens: Vec<&str> = text.split_whitespace().collect();
        for t in tokens {
            self.index
                .entry(t.to_string())
                .or_insert(vec![])
                .push(id.to_string());
        }
    }

    pub fn search(&self, query: &str) -> Vec<String> {
        let tokens: Vec<&str> = query.split_whitespace().collect();
        let mut result: Vec<String> = vec![];

        for t in tokens {
            if let Some(ids) = self.index.get(t) {
                for id in ids {
                    result.push(id.clone());
                }
            }
        }
        result
    }
}
