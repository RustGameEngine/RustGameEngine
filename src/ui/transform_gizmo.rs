use crate::game_object::GameObject;

pub struct TransformGizmo {
    pub mode: GizmoMode,
}

pub enum GizmoMode {
    Move,
    Scale,
    Rotate,
}

impl TransformGizmo {
    pub fn new() -> Self {
        Self {
            mode: GizmoMode::Move, // Default to Move mode
        }
    }

    pub fn render(
        &self,
        ui: &mut egui::Ui,
        selected_object: &mut Option<GameObject>,
        camera_position: [f32; 2],
        zoom: f32,
    ) {
        if let Some(object) = selected_object {
            let object_screen_position = [
                camera_position[0] + object.position[0] * zoom,
                camera_position[1] + object.position[1] * zoom,
            ];

            match self.mode {
                GizmoMode::Move => {
                    ui.horizontal(|ui| {
                        ui.label("Move Gizmo:");
                        ui.add(egui::DragValue::new(&mut object.position[0]).speed(0.1));
                        ui.add(egui::DragValue::new(&mut object.position[1]).speed(0.1));
                    });
                }
                GizmoMode::Scale => {
                    ui.horizontal(|ui| {
                        ui.label("Scale Gizmo:");
                        ui.add(egui::DragValue::new(&mut object.scale[0]).speed(0.1));
                        ui.add(egui::DragValue::new(&mut object.scale[1]).speed(0.1));
                        ui.add(egui::DragValue::new(&mut object.scale[2]).speed(0.1));
                    });
                }
                GizmoMode::Rotate => {
                    ui.horizontal(|ui| {
                        ui.label("Rotate Gizmo:");
                        ui.add(egui::DragValue::new(&mut object.rotation[0]).speed(0.1));
                        ui.add(egui::DragValue::new(&mut object.rotation[1]).speed(0.1));
                        ui.add(egui::DragValue::new(&mut object.rotation[2]).speed(0.1));
                    });
                }
            }
        }
    }
}