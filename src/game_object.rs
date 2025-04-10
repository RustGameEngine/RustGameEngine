#[derive(Clone)]
pub struct GameObject {
    pub id: usize,
    pub name: String,
    pub parent_id: Option<usize>,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub visible: bool,
}

impl GameObject {
    pub fn new(id: usize, name: String, position: [f32; 3], rotation: [f32; 3]) -> Self {
        Self {
            id,
            name,
            parent_id: None,
            position,
            rotation,
            scale: [1.0, 1.0, 1.0], // Default scale value
            visible: true,           // Default visibility value
        }
    }
}
