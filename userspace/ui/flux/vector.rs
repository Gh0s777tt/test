pub struct VectorPath {
    pub points: Vec<(f32, f32)>,
}

impl VectorPath {
    pub fn scale(&self, factor: f32) -> Self {
        Self {
            points: self.points.iter()
                .map(|(x, y)| (x * factor, y * factor))
                .collect(),
        }
    }
}
