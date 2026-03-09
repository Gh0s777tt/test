pub struct Model {
    pub name: String,
    pub path: String,
    pub context_size: usize,
}

impl Model {
    pub fn load_default() -> Self {
        Self {
            name: "Vantis-Cortex-7B".into(),
            path: "/models/cortex.gguf".into(),
            context_size: 4096,
        }
    }
}
