use super::index::SemanticIndex;

pub fn search(index: &SemanticIndex, query: Vec<f32>) -> Option<String> {
    index.data.keys().next().cloned()
}
