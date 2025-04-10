pub struct Camera {
    pub position: [f32; 2],
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            zoom: 1.0,
        }
    }
}
