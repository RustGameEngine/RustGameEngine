use crate::game_object::GameObject;

pub fn render_scene_view_with_context_menu(
    ui: &mut egui::Ui,
    objects: &mut Vec<GameObject>,
    selected_object: &mut Option<GameObject>,
    camera_position: [f32; 2],
    zoom: f32,
) {
    let scene_rect = ui.available_rect_before_wrap();

    // Render gridlines
    let grid_spacing = 50.0 * zoom; // Adjust grid spacing with zoom level
    let start_x = (scene_rect.min.x - camera_position[0] % grid_spacing).floor();
    let start_y = (scene_rect.min.y - camera_position[1] % grid_spacing).floor();

    let painter = ui.painter();
    for x in (start_x as i32..scene_rect.max.x as i32).step_by(grid_spacing as usize) {
        painter.line_segment(
            [
                egui::pos2(x as f32, scene_rect.min.y),
                egui::pos2(x as f32, scene_rect.max.y),
            ],
            egui::Stroke::new(1.0, egui::Color32::GRAY),
        );
    }
    for y in (start_y as i32..scene_rect.max.y as i32).step_by(grid_spacing as usize) {
        painter.line_segment(
            [
                egui::pos2(scene_rect.min.x, y as f32),
                egui::pos2(scene_rect.max.x, y as f32),
            ],
            egui::Stroke::new(1.0, egui::Color32::GRAY),
        );
    }

    // Draw axis lines
    painter.line_segment(
        [
            egui::pos2(scene_rect.min.x, scene_rect.center().y),
            egui::pos2(scene_rect.max.x, scene_rect.center().y),
        ],
        egui::Stroke::new(2.0, egui::Color32::RED), // X-axis
    );
    painter.line_segment(
        [
            egui::pos2(scene_rect.center().x, scene_rect.min.y),
            egui::pos2(scene_rect.center().x, scene_rect.max.y),
        ],
        egui::Stroke::new(2.0, egui::Color32::GREEN), // Y-axis
    );

    // Render objects
    for object in objects.iter_mut() {
        let object_screen_position = [
            scene_rect.min.x + (camera_position[0] + object.position[0] * zoom),
            scene_rect.min.y + (camera_position[1] + object.position[1] * zoom),
        ];

        let color = if let Some(selected) = selected_object {
            if selected.id == object.id {
                egui::Color32::LIGHT_BLUE // Highlight selected object
            } else {
                egui::Color32::WHITE
            }
        } else {
            egui::Color32::WHITE
        };

        if ui.put(
            egui::Rect::from_min_size(
                egui::pos2(object_screen_position[0], object_screen_position[1]),
                egui::vec2(40.0, 40.0),
            ),
            egui::Button::new(&object.name).fill(color),
        )
        .clicked()
        {
            *selected_object = Some(object.clone());
        }
    }

    // Context menu
    if ui.ctx().input().pointer.secondary_clicked() {
        egui::menu::menu_button(ui, "Context Menu", |ui| {
            if ui.button("Create Cube").clicked() {
                let new_object = GameObject::new(
                    objects.len() + 1,
                    "New Cube".to_string(),
                    [0.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0],
                );
                objects.push(new_object);
                println!("Created new cube.");
            }

            if let Some(selected) = selected_object {
                if ui.button("Delete Selected").clicked() {
                    objects.retain(|obj| obj.id != selected.id);
                    *selected_object = None;
                    println!("Deleted selected object.");
                }
            }
        });
    }
}
