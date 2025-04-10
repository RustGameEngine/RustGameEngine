// src/ui/panel.rs

/// Represents a generic UI panel
pub struct Panel {
    pub title: String,
    pub content: Box<dyn Fn()>, // Content rendering logic
}

impl Panel {
    /// Create a new panel with a given title and content
    pub fn new(title: String, content: Box<dyn Fn()>) -> Self {
        Self { title, content }
    }

    /// Render the panel (for now, we'll print its content to the console)
    pub fn render(&self) {
        println!("--- Panel: {} ---", self.title);
        (self.content)();
    }
}
