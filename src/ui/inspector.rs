use crate::game_object::GameObject;

pub struct InspectorPanel {
    pub title: String,
}

impl InspectorPanel {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, selected_object: &mut Option<GameObject>) {
        render_inspector_panel(ui, selected_object);
    }    
}

pub fn render_inspector_panel(
    ui: &mut egui::Ui,
    selected_object: &mut Option<GameObject>,
) {
    if let Some(object) = selected_object {
        ui.label("Inspector");
        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut object.name);
        });

        ui.horizontal(|ui| {
            ui.label("Position:");
            for i in 0..3 {
                ui.add(egui::DragValue::new(&mut object.position[i]).speed(0.1));
            }
        });

        ui.horizontal(|ui| {
            ui.label("Rotation:");
            for i in 0..3 {
                ui.add(egui::DragValue::new(&mut object.rotation[i]).speed(0.1));
            }
        });

        ui.horizontal(|ui| {
            ui.label("Scale:");
            for i in 0..3 {
                ui.add(egui::DragValue::new(&mut object.scale[i]).speed(0.1));
            }
        });

        ui.horizontal(|ui| {
            ui.label("Visible:");
            ui.checkbox(&mut object.visible, "Yes");
        });
    } else {
        ui.label("No object selected.");
    }
}
