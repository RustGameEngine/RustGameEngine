pub struct Menu {
    pub title: String,
    pub items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(title: String, items: Vec<MenuItem>) -> Self {
        Self { title, items }
    }

    // Add the render method
    pub fn render(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(&self.title);
            for item in &self.items {
                if ui.button(&item.label).clicked() {
                    (item.action)(); // Execute the action tied to the menu item
                }
            }
        });
    }
}

pub struct MenuItem {
    pub label: String,
    pub action: Box<dyn Fn()>,
}

impl MenuItem {
    pub fn new(label: String, action: Box<dyn Fn()>) -> Self {
        Self { label, action }
    }
}
