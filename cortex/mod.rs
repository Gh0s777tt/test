pub mod llm;
pub mod semantic;
pub mod automation;
pub mod semantic_search;

#[cfg(test)]
mod tests {
    use super::automation::intent::{parse, Intent};
    use super::semantic_search::SemanticIndex;

    #[test]
    fn test_intent_parser_shutdown() {
        let intent = parse("proszę wyłącz system");
        assert!(matches!(intent, Intent::Shutdown));
    }

    #[test]
    fn test_semantic_index_search() {
        let mut index = SemanticIndex::new();
        index.add_document("doc-1", "vantis cortex semantic search");
        let hits = index.search("semantic");
        assert!(hits.iter().any(|id| id == "doc-1"));
    }
}
