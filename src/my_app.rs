use crate::{
    camera::Camera,
    cube::Cube,
    game_object::GameObject,
    ui::{
        hierarchy::SceneHierarchyPanel,
        inspector::InspectorPanel,
        menu::{Menu, MenuItem},
        scene_view::render_scene_view_with_context_menu,
        camera::camera_controls::{pan_camera, zoom_camera},
    },
};
use egui::{Context, Rect, Ui};

pub struct MyApp {
    pub menu: Menu,
    pub hierarchy_panel: SceneHierarchyPanel,
    pub inspector_panel: InspectorPanel,
    pub selected_object: Option<GameObject>,
    pub camera: Camera,
    pub cube: Option<Cube>,
    pub show_cube: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            menu: Menu::new(
                "Main Menu".to_string(),
                vec![
                    MenuItem::new("Start Game".to_string(), Box::new(|| println!("Game Started"))),
                    MenuItem::new("Exit".to_string(), Box::new(|| std::process::exit(0))),
                ],
            ),
            hierarchy_panel: SceneHierarchyPanel::new(
                "".to_string(),
                vec![
                    GameObject::new(1, "Root Object".to_string(), [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
                    GameObject::new(2, "Child Object 1".to_string(), [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
                    GameObject::new(3, "Child Object 2".to_string(), [2.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
                ],
            ),
            inspector_panel: InspectorPanel::new("".to_string()),
            selected_object: None,
            camera: Camera::default(),
            cube: Some(Cube::new([100.0, 100.0], 50.0, egui::Color32::RED)), // Default cube
            show_cube: true, // Cube is enabled by default
        }
    }
}

impl MyApp {
    pub fn render_gridlines(&self, ui: &mut Ui, scene_rect: Rect) {
        let grid_spacing = 50.0 * self.camera.zoom; // Adjust grid spacing with zoom level
        let start_x = (scene_rect.min.x - self.camera.position[0] % grid_spacing).floor();
        let start_y = (scene_rect.min.y - self.camera.position[1] % grid_spacing).floor();

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
    }

    pub fn render_objects(&self, ui: &mut Ui, scene_rect: Rect) {
        for object in &self.hierarchy_panel.objects {
            let object_position = [
                scene_rect.min.x + (self.camera.position[0] + object.position[0] * self.camera.zoom),
                scene_rect.min.y + (self.camera.position[1] + object.position[1] * self.camera.zoom),
            ];
            let object_size = 50.0 * self.camera.zoom; // Example size for objects

            // Render the object
            ui.painter().rect_filled(
                egui::Rect::from_min_size(
                    egui::pos2(object_position[0], object_position[1]),
                    egui::vec2(object_size, object_size),
                ),
                egui::Rounding::none(),
                egui::Color32::WHITE,
            );

            // Highlight the selected object
            if let Some(selected) = &self.selected_object {
                if selected.id == object.id {
                    ui.painter().rect_stroke(
                        egui::Rect::from_min_size(
                            egui::pos2(object_position[0], object_position[1]),
                            egui::vec2(object_size, object_size),
                        ),
                        egui::Rounding::none(),
                        egui::Stroke::new(2.0, egui::Color32::YELLOW),
                    );
                }
            }
        }
    }

    pub fn render_cube_controls(&mut self, ui: &mut Ui) {
        if let Some(cube) = &mut self.cube {
            ui.label("Cube Properties:");
            ui.horizontal(|ui| {
                ui.label("Position:");
                ui.add(egui::DragValue::new(&mut cube.position[0]).speed(1.0));
                ui.add(egui::DragValue::new(&mut cube.position[1]).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Size:");
                ui.add(egui::DragValue::new(&mut cube.size).speed(1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Color:");
                let mut color = [
                    cube.color.r() as f32 / 255.0,
                    cube.color.g() as f32 / 255.0,
                    cube.color.b() as f32 / 255.0,
                ];
                if ui.color_edit_button_rgb(&mut color).changed() {
                    cube.color = egui::Color32::from_rgb(
                        (color[0] * 255.0) as u8,
                        (color[1] * 255.0) as u8,
                        (color[2] * 255.0) as u8,
                    );
                }
            });
            ui.checkbox(&mut self.show_cube, "Show Cube");
        } else {
            ui.label("No cube available.");
        }
    }

    pub fn render_cube(&self, ui: &mut Ui, scene_rect: Rect) {
        if let Some(cube) = &self.cube {
            cube.render(ui, &self.camera, scene_rect);

            // Highlight the cube if selected
            if self.selected_object.is_some() {
                let cube_position = [
                    scene_rect.min.x + (self.camera.position[0] + cube.position[0] * self.camera.zoom),
                    scene_rect.min.y + (self.camera.position[1] + cube.position[1] * self.camera.zoom),
                ];
                let cube_size = cube.size * self.camera.zoom;

                ui.painter().rect_stroke(
                    egui::Rect::from_min_size(
                        egui::pos2(cube_position[0], cube_position[1]),
                        egui::vec2(cube_size, cube_size),
                    ),
                    egui::Rounding::none(),
                    egui::Stroke::new(2.0, egui::Color32::YELLOW),
                );
            }
        }
    }

    pub fn handle_camera_controls(&mut self, ctx: &Context) {
        let input = ctx.input();

        // Panning with right mouse button
        if input.pointer.secondary_down() {
            if let Some(pointer_pos) = input.pointer.interact_pos() {
                if let Some(last_pos) = input.pointer.interact_pos() {
                    let delta = pointer_pos - last_pos;
                    let zoom = self.camera.zoom;
                    pan_camera(&mut self.camera, -delta.x / zoom, -delta.y / zoom);
                }
            }
        }

        // Zooming with scroll wheel
        if input.scroll_delta.y != 0.0 {
            zoom_camera(&mut self.camera, 1.0 + input.scroll_delta.y * 0.01);
            self.camera.zoom = self.camera.zoom.clamp(0.1, 10.0); // Clamp zoom level
        }
    }

    pub fn handle_object_selection(&mut self, ctx: &Context, scene_rect: Rect) {
        if let Some(pointer_pos) = ctx.input().pointer.hover_pos() {
            if scene_rect.contains(pointer_pos) && ctx.input().pointer.any_click() {
                for object in &self.hierarchy_panel.objects {
                    let object_position = [
                        scene_rect.min.x + (self.camera.position[0] + object.position[0] * self.camera.zoom),
                        scene_rect.min.y + (self.camera.position[1] + object.position[1] * self.camera.zoom),
                    ];
                    let object_size = 50.0 * self.camera.zoom;

                    let object_rect = egui::Rect::from_min_size(
                        egui::pos2(object_position[0], object_position[1]),
                        egui::vec2(object_size, object_size),
                    );

                    if object_rect.contains(pointer_pos) {
                        self.selected_object = Some(object.clone());
                        return;
                    }
                }
                self.selected_object = None; // Deselect if no object is clicked
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle camera controls
        self.handle_camera_controls(ctx);

        // Top panel for the menu
        egui::TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            self.menu.render(ui);
        });

        // Scene Hierarchy Panel
        egui::SidePanel::left("hierarchy_panel").show(ctx, |ui| {
            self.hierarchy_panel.render(ui, &mut self.selected_object);
        });

        // Inspector Panel
        egui::SidePanel::right("inspector_panel").show(ctx, |ui| {
            self.inspector_panel.render(ui, &mut self.selected_object);
        });

        // Extract mutable references and required data to avoid conflicting borrows
        let objects = &mut self.hierarchy_panel.objects;
        let selected_object = &mut self.selected_object;
        let camera_position = self.camera.position;
        let camera_zoom = self.camera.zoom;

        // Scene View with Context Menu
        egui::CentralPanel::default().show(ctx, |ui| {
            render_scene_view_with_context_menu(
                ui,
                objects,
                selected_object,
                camera_position,
                camera_zoom,
            );
        });
    }
}