use crate::game_object::GameObject;

pub struct SceneHierarchyPanel {
    pub title: String,
    pub objects: Vec<GameObject>,        // List of objects in the hierarchy
    pub dragged_object: Option<usize>,  // Index of the currently dragged object
}

impl SceneHierarchyPanel {
    pub fn new(title: String, objects: Vec<GameObject>) -> Self {
        Self {
            title,
            objects,
            dragged_object: None, // Initialize with no dragged object
        }
    }
    pub fn render(&mut self, ui: &mut egui::Ui, selected_object: &mut Option<GameObject>) {
        ui.heading(&self.title);
        ui.separator();
    
        if self.objects.is_empty() {
            ui.label("No objects in the hierarchy.");
            return;
        }
    
        for object in &self.objects {
            ui.horizontal(|ui| {
                let response = ui.button(&object.name);
    
                if response.clicked() {
                    *selected_object = Some(object.clone());
                    println!("Selected object: {}", object.name);
                }
            });
        }
    }
    
    fn render_object_tree(
        &mut self,
        ui: &mut egui::Ui,
        object: &GameObject,
        selected_object: &mut Option<GameObject>,
        indent_level: usize,
    ) {
        ui.horizontal(|ui| {
            // Adjust indentation based on hierarchy level
            for _ in 0..indent_level {
                ui.label("    "); // Indentation for child objects
            }
    
            // Object buttons
            let response = ui.button(&object.name);
    
            if response.clicked() {
                *selected_object = Some(object.clone());
                println!("Selected object: {}", object.name);
            }
    
            // Drag-and-drop logic
            if response.dragged() {
                self.dragged_object = Some(object.id);
            }
    
            if response.hovered() && ui.button("Drop Here").clicked() {
                if let Some(dragged_id) = self.dragged_object {
                    if let Some(dragged_object) = self.objects.iter_mut().find(|obj| obj.id == dragged_id) {
                        dragged_object.parent_id = Some(object.id);
                        self.dragged_object = None;
                        println!("Dropped object {} onto {}", dragged_object.name, object.name);
                    }
                }
            }
        });
    
        // Recursively render child objects
        let child_ids: Vec<usize> = self.objects
            .iter()
            .filter(|obj| obj.parent_id == Some(object.id))
            .map(|obj| obj.id)
            .collect();
    
        for child_id in child_ids {
            if let Some(child_object) = self.objects.iter().find(|obj| obj.id == child_id).cloned() {
                self.render_object_tree(ui, &child_object, selected_object, indent_level + 1);
            }
        }
    }
    
            
}
