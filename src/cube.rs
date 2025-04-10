use egui::{Color32, Rect, Ui};

use crate::camera::Camera;

pub struct Cube {
    pub position: [f32; 2],
    pub size: f32,
    pub color: Color32,
}

impl Cube {
    pub fn new(position: [f32; 2], size: f32, color: Color32) -> Self {
        Self {
            position,
            size,
            color,
        }
    }

    pub fn render(&self, ui: &mut Ui, camera: &Camera, scene_rect: Rect) {
        let cube_position = [
            scene_rect.min.x + (camera.position[0] + self.position[0] * camera.zoom),
            scene_rect.min.y + (camera.position[1] + self.position[1] * camera.zoom),
        ];

        ui.painter().rect_filled(
            Rect::from_min_size(
                egui::pos2(cube_position[0], cube_position[1]),
                egui::vec2(self.size * camera.zoom, self.size * camera.zoom),
            ),
            egui::Rounding::none(),
            self.color,
        );
    }
}
