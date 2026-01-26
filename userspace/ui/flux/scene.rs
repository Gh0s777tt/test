pub struct SceneNode {
    pub children: Vec<SceneNode>,
    pub opacity: f32,
}

impl SceneNode {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            opacity: 1.0,
        }
    }
}
