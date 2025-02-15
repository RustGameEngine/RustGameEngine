use raylib::prelude::*;

pub struct Panels {
    pub bottom_panel_height: i32,
    pub right_panel_width: i32,
    pub left_panel_width: i32,
    pub resizing_bottom: bool,
    pub resizing_right: bool,
    pub resizing_left: bool,
}

impl Panels {
    pub fn new(bottom_panel_height: i32, right_panel_width: i32, left_panel_width: i32) -> Self {
        Panels {
            bottom_panel_height,
            right_panel_width,
            left_panel_width,
            resizing_bottom: false,
            resizing_right: false,
            resizing_left: false,
        }
    }

    pub fn handle_resizing(&mut self, d: &mut RaylibDrawHandle, screen_width: i32, screen_height: i32, menu_bar_height: i32) {
        let bottom_panel_y = screen_height - self.bottom_panel_height;
        let right_panel_x = screen_width - self.right_panel_width;

        // Resizing logic for bottom panel
        let resize_bar_bottom = Rectangle::new(0.0, bottom_panel_y as f32 - 5.0, screen_width as f32, 10.0);
        d.draw_rectangle_rec(resize_bar_bottom, Color::GRAY);

        let mouse = d.get_mouse_position();
        if resize_bar_bottom.check_collision_point_rec(mouse) {
            d.draw_text("Resize", 10, bottom_panel_y - 20, 14, Color::BLACK);
            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                self.resizing_bottom = true;
            }
        }

        if self.resizing_bottom {
            if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                self.bottom_panel_height = (screen_height - mouse.y as i32).clamp(100, screen_height / 2);
            } else {
                self.resizing_bottom = false;
            }
        }

        // Resizing logic for right panel
        let resize_bar_right = Rectangle::new(right_panel_x as f32 - 5.0, menu_bar_height as f32, 10.0, screen_height as f32 - menu_bar_height as f32);
        d.draw_rectangle_rec(resize_bar_right, Color::GRAY);

        if resize_bar_right.check_collision_point_rec(mouse) {
            d.draw_text("Resize", right_panel_x - 50, 10, 14, Color::BLACK);
            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                self.resizing_right = true;
            }
        }

        if self.resizing_right {
            if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                self.right_panel_width = (screen_width - mouse.x as i32).clamp(100, screen_width / 2);
            } else {
                self.resizing_right = false;
            }
        }

        // Resizing logic for left panel
        let resize_bar_left = Rectangle::new(self.left_panel_width as f32 - 5.0, menu_bar_height as f32, 10.0, (screen_height - menu_bar_height - self.bottom_panel_height) as f32);
        d.draw_rectangle_rec(resize_bar_left, Color::GRAY);

        if resize_bar_left.check_collision_point_rec(mouse) {
            d.draw_text("Resize", self.left_panel_width - 50, 10, 14, Color::BLACK);
            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
                self.resizing_left = true;
            }
        }

        if self.resizing_left {
            if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
                self.left_panel_width = (mouse.x as i32).clamp(100, screen_width / 2);
            } else {
                self.resizing_left = false;
            }
        }
    }
}
