use raylib::prelude::*;
use std::sync::{Arc, Mutex};
use crate::font::FontManager;

pub struct UI {
    pub selected_tab: &'static str,
    pub show_favourites: bool,
    pub show_assets: bool,
    pub show_packages: bool,
    pub show_context_menu: bool,
    pub context_menu_position: Vector2,
    pub font_manager: FontManager,
    pub show_right_context_menu: bool,
    pub show_left_context_menu: bool,
    pub show_assets_context_menu: bool,
}

impl UI {
    pub fn new(font_manager: FontManager) -> Self {
        UI {
            selected_tab: "Console",
            show_favourites: true,
            show_assets: true,
            show_packages: true,
            show_context_menu: false,
            show_right_context_menu: false,
            show_left_context_menu: false,
            show_assets_context_menu: false,
            context_menu_position: Vector2::zero(),
            font_manager,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, screen_width: i32, screen_height: i32, bottom_panel_height: i32, right_panel_width: i32, left_panel_width: i32, menu_bar_height: i32, console_log: &Arc<Mutex<Vec<String>>>) {
        // Adjust bottom panel bounds
        let bottom_panel_y = screen_height - bottom_panel_height;
        let bottom_panel_width = screen_width - right_panel_width;
        let bottom_panel_rect = Rectangle::new(0.0, bottom_panel_y as f32, bottom_panel_width as f32, bottom_panel_height as f32);

        // Adjust right panel bounds
        let right_panel_x = screen_width - right_panel_width;
        let right_panel_rect = Rectangle::new(right_panel_x as f32, menu_bar_height as f32, right_panel_width as f32, screen_height as f32 - menu_bar_height as f32);

        // Adjust left panel bounds
        let left_panel_rect = Rectangle::new(0.0, menu_bar_height as f32, left_panel_width as f32, screen_height as f32 - menu_bar_height as f32);

        // Adjust middle panel bounds
        let middle_panel_rect = Rectangle::new(left_panel_width as f32, menu_bar_height as f32, (screen_width - left_panel_width - right_panel_width) as f32, (screen_height - menu_bar_height - bottom_panel_height) as f32);

        // Draw bottom panel
        d.draw_rectangle_rec(bottom_panel_rect, Color::DARKGRAY);

        // Draw right panel
        d.draw_rectangle_rec(right_panel_rect, Color::DARKGRAY);

        // Draw left panel
        d.draw_rectangle_rec(left_panel_rect, Color::DARKGRAY);

        // Draw middle panel
        d.draw_rectangle_rec(middle_panel_rect, Color::BLACK);

        // Top Menu Bar
        let menu_file = Rectangle::new(10.0, 5.0, 80.0, menu_bar_height as f32 - 10.0);
        let menu_edit = Rectangle::new(100.0, 5.0, 80.0, menu_bar_height as f32 - 10.0);
        let menu_assets = Rectangle::new(190.0, 5.0, 80.0, menu_bar_height as f32 - 10.0);
        let menu_gameobject = Rectangle::new(280.0, 5.0, 120.0, menu_bar_height as f32 - 10.0);
        let menu_component = Rectangle::new(410.0, 5.0, 120.0, menu_bar_height as f32 - 10.0);
        let menu_services = Rectangle::new(540.0, 5.0, 100.0, menu_bar_height as f32 - 10.0);
        let menu_window = Rectangle::new(650.0, 5.0, 100.0, menu_bar_height as f32 - 10.0);
        let menu_help = Rectangle::new(760.0, 5.0, 80.0, menu_bar_height as f32 - 10.0);

        // Draw Menu Bar
        d.draw_rectangle(0, 0, screen_width, menu_bar_height, Color::WHITE);

        self.font_manager.draw_text(d, "File", Vector2::new(menu_file.x + 10.0, menu_file.y + 8.0), 16.0, 1.0, Color::BLACK);
        self.font_manager.draw_text(d, "Edit", Vector2::new(menu_edit.x + 10.0, menu_edit.y + 8.0), 16.0, 1.0, Color::BLACK);
        self.font_manager.draw_text(d, "Assets", Vector2::new(menu_assets.x + 10.0, menu_assets.y + 8.0), 16.0, 1.0, Color::BLACK);
        self.font_manager.draw_text(d, "GameObject", Vector2::new(menu_gameobject.x + 10.0, menu_gameobject.y + 8.0), 16.0, 1.0, Color::BLACK);
        self.font_manager.draw_text(d, "Component", Vector2::new(menu_component.x + 10.0, menu_component.y + 8.0), 16.0, 1.0, Color::BLACK);
        self.font_manager.draw_text(d, "Services", Vector2::new(menu_services.x + 10.0, menu_services.y + 8.0), 16.0, 1.0, Color::BLACK);
        self.font_manager.draw_text(d, "Window", Vector2::new(menu_window.x + 10.0, menu_window.y + 8.0), 16.0, 1.0, Color::BLACK);
        self.font_manager.draw_text(d, "Help", Vector2::new(menu_help.x + 10.0, menu_help.y + 8.0), 16.0, 1.0, Color::BLACK);

        // Add right-click detection for right_panel_rect
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) && right_panel_rect.check_collision_point_rec(d.get_mouse_position()) {
            self.show_right_context_menu = true;
            self.show_left_context_menu = false;
            self.show_assets_context_menu = false;
            self.context_menu_position = d.get_mouse_position();
        }

        // Add right-click detection for left_panel_rect
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) && left_panel_rect.check_collision_point_rec(d.get_mouse_position()) {
            self.show_right_context_menu = false;
            self.show_left_context_menu = true;
            self.show_assets_context_menu = false;
            self.context_menu_position = d.get_mouse_position();
        }

        // Add right-click detection for bottom_panel_rect (within Assets Content)
        let assets_content_rect = Rectangle::new(10.0, bottom_panel_y as f32 + 30.0 + 10.0, bottom_panel_width as f32 - 20.0, bottom_panel_height as f32 - 30.0 - 20.0);
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) && assets_content_rect.check_collision_point_rec(d.get_mouse_position()) {
            self.show_right_context_menu = false;
            self.show_left_context_menu = false;
            self.show_assets_context_menu = true;
            self.context_menu_position = d.get_mouse_position();
        }

        // Draw context menu for right panel
        if self.show_right_context_menu {
            let right_menu_items = [
                "Right Option 1", "Right Option 2", "Right Option 3", "Right Option 4"
            ];

            let max_text_width = right_menu_items.iter().map(|item| self.font_manager.measure_text(item, 16.0, 1.0).x as i32).max().unwrap_or(0);
            let context_menu_width = (max_text_width as f32 * 1.1).ceil();
            let context_menu_height = (right_menu_items.len() as f32 * 20.0 * 1.1).ceil();

            // Adjust context menu position to stay within right panel bounds
            let mut context_menu_x = self.context_menu_position.x;
            let mut context_menu_y = self.context_menu_position.y;
            if context_menu_x + context_menu_width > right_panel_rect.x + right_panel_rect.width {
                context_menu_x = right_panel_rect.x + right_panel_rect.width - context_menu_width;
            }
            if context_menu_y + context_menu_height > right_panel_rect.y + right_panel_rect.height {
                context_menu_y = right_panel_rect.y + right_panel_rect.height - context_menu_height;
            }

            let context_menu_rect = Rectangle::new(context_menu_x, context_menu_y, context_menu_width, context_menu_height);
            d.draw_rectangle_rec(context_menu_rect, Color::LIGHTGRAY);

            for (i, item) in right_menu_items.iter().enumerate() {
                let item_rect = Rectangle::new(context_menu_rect.x, context_menu_rect.y + i as f32 * 20.0, context_menu_rect.width, 20.0);
                d.draw_rectangle_lines_ex(item_rect, 1.0, Color::BLACK);
                self.font_manager.draw_text(d, item, Vector2::new(item_rect.x + 10.0, item_rect.y + 2.0), 16.0, 1.0, Color::BLACK);
            }

            // Hide context menu if left-click is detected outside the menu
            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && !context_menu_rect.check_collision_point_rec(d.get_mouse_position()) {
                self.show_right_context_menu = false;
            }
        }
        // Draw context menu for left panel
        if self.show_left_context_menu {
            let left_menu_items = [
                "Left Option 1", "Left Option 2", "Left Option 3", "Left Option 4"
            ];

            let max_text_width = left_menu_items.iter().map(|item| self.font_manager.measure_text(item, 16.0, 1.0).x as i32).max().unwrap_or(0);
            let context_menu_width = (max_text_width as f32 * 1.1).ceil();
            let context_menu_height = (left_menu_items.len() as f32 * 20.0 * 1.1).ceil();

            // Adjust context menu position to stay within left panel bounds
            let mut context_menu_x = self.context_menu_position.x;
            let mut context_menu_y = self.context_menu_position.y;
            if context_menu_x + context_menu_width > left_panel_rect.x + left_panel_rect.width {
                context_menu_x = left_panel_rect.x + left_panel_rect.width - context_menu_width;
            }
            if context_menu_y + context_menu_height > left_panel_rect.y + left_panel_rect.height {
                context_menu_y = left_panel_rect.y + left_panel_rect.height - context_menu_height;
            }

            let context_menu_rect = Rectangle::new(context_menu_x, context_menu_y, context_menu_width, context_menu_height);
            d.draw_rectangle_rec(context_menu_rect, Color::LIGHTGRAY);

            for (i, item) in left_menu_items.iter().enumerate() {
                let item_rect = Rectangle::new(context_menu_rect.x, context_menu_rect.y + i as f32 * 20.0, context_menu_rect.width, 20.0);
                d.draw_rectangle_lines_ex(item_rect, 1.0, Color::BLACK);
                self.font_manager.draw_text(d, item, Vector2::new(item_rect.x + 10.0, item_rect.y + 2.0), 16.0, 1.0, Color::BLACK);
            }

            // Hide context menu if left-click is detected outside the menu
            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && !context_menu_rect.check_collision_point_rec(d.get_mouse_position()) {
                self.show_left_context_menu = false;
            }
        }                        
        // Tab Bar
        let tab_bar_height = 30;
        let tab_console = Rectangle::new(10.0, bottom_panel_y as f32 + 5.0, 80.0, tab_bar_height as f32);
        let tab_assets = Rectangle::new(100.0, bottom_panel_y as f32 + 5.0, 80.0, tab_bar_height as f32);

        // Draw Tabs
        d.draw_rectangle_rec(tab_console, if self.selected_tab == "Console" { Color::LIGHTGRAY } else { Color::GRAY });
        d.draw_rectangle_rec(tab_assets, if self.selected_tab == "Assets" { Color::LIGHTGRAY } else { Color::GRAY });

        self.font_manager.draw_text(d, "Console", Vector2::new(tab_console.x + 10.0, tab_console.y + 8.0), 16.0, 1.0, Color::BLACK);
        self.font_manager.draw_text(d, "Assets", Vector2::new(tab_assets.x + 10.0, tab_assets.y + 8.0), 16.0, 1.0, Color::BLACK);

        // Tab Switching Logic
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse = d.get_mouse_position();
            if tab_console.check_collision_point_rec(mouse) {
                self.selected_tab = "Console";
            } else if tab_assets.check_collision_point_rec(mouse) {
                self.selected_tab = "Assets";
            }
        }

        // Draw active tab content
        let content_rect = Rectangle::new(10.0, bottom_panel_y as f32 + tab_bar_height as f32 + 10.0, bottom_panel_width as f32 - 20.0, bottom_panel_height as f32 - tab_bar_height as f32 - 20.0);
        d.draw_rectangle_rec(content_rect, Color::BLACK);

        if self.selected_tab == "Console" {
            let log = console_log.lock().unwrap();
            for (i, line) in log.iter().enumerate() {
                self.font_manager.draw_text(d, line, Vector2::new(content_rect.x + 10.0, content_rect.y + 10.0 + i as f32 * 20.0), 16.0, 1.0, Color::WHITE);
            }
        } else {
            // Split the Assets panel into two sections
            let assets_panel_left_width = content_rect.width * 0.2;
            let assets_panel_right_width = content_rect.width * 0.8;
            let assets_panel_left = Rectangle::new(content_rect.x, content_rect.y, assets_panel_left_width, content_rect.height);
            let assets_panel_right = Rectangle::new(content_rect.x + assets_panel_left_width, content_rect.y, assets_panel_right_width, content_rect.height);

            // Draw left section of Assets panel
            d.draw_rectangle_rec(assets_panel_left, Color::DARKGRAY);

            // Favourites menu
            let favourites_rect = Rectangle::new(assets_panel_left.x + 10.0, assets_panel_left.y + 10.0, 150.0, 20.0);
            self.font_manager.draw_text(d, if self.show_favourites { "▼ Favourites" } else { "▶ Favourites" }, Vector2::new(favourites_rect.x, favourites_rect.y), 16.0, 1.0, Color::WHITE);
            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && favourites_rect.check_collision_point_rec(d.get_mouse_position()) {
                self.show_favourites = !self.show_favourites;
            }
            if self.show_favourites {
                self.font_manager.draw_text(d, "  All Materials", Vector2::new(favourites_rect.x + 10.0, favourites_rect.y + 20.0), 16.0, 1.0, Color::WHITE);
                self.font_manager.draw_text(d, "  All Models", Vector2::new(favourites_rect.x + 10.0, favourites_rect.y + 40.0), 16.0, 1.0, Color::WHITE);
                self.font_manager.draw_text(d, "  All Prefabs", Vector2::new(favourites_rect.x + 10.0, favourites_rect.y + 60.0), 16.0, 1.0, Color::WHITE);
            }

            // Assets menu
            let assets_rect = Rectangle::new(assets_panel_left.x + 10.0, favourites_rect.y + if self.show_favourites { 80.0 } else { 20.0 }, 150.0, 20.0);
            self.font_manager.draw_text(d, if self.show_assets { "▼ Assets" } else { "▶ Assets" }, Vector2::new(assets_rect.x, assets_rect.y), 16.0, 1.0, Color::WHITE);
            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && assets_rect.check_collision_point_rec(d.get_mouse_position()) {
                self.show_assets = !self.show_assets;
            }
            if self.show_assets {
                self.font_manager.draw_text(d, "  Scenes", Vector2::new(assets_rect.x + 10.0, assets_rect.y + 20.0), 16.0, 1.0, Color::WHITE);
                self.font_manager.draw_text(d, "  Scripts", Vector2::new(assets_rect.x + 10.0, assets_rect.y + 40.0), 16.0, 1.0, Color::WHITE);
            }

            // Packages menu
            let packages_rect = Rectangle::new(assets_panel_left.x + 10.0, assets_rect.y + if self.show_assets { 60.0 } else { 20.0 }, 150.0, 20.0);
            self.font_manager.draw_text(d, if self.show_packages { "▼ Packages" } else { "▶ Packages" }, Vector2::new(packages_rect.x, packages_rect.y), 16.0, 1.0, Color::WHITE);
            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && packages_rect.check_collision_point_rec(d.get_mouse_position()) {
                self.show_packages = !self.show_packages;
            }

            // Draw right section of Assets panel
            d.draw_rectangle_rec(assets_panel_right, Color::LIGHTGRAY);
            self.font_manager.draw_text(d, "Assets Content", Vector2::new(assets_panel_right.x + 10.0, assets_panel_right.y + 10.0), 16.0, 1.0, Color::BLACK);

            // Context menu logic
            if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT) && assets_panel_right.check_collision_point_rec(d.get_mouse_position()) {
                self.show_context_menu = true;
                self.context_menu_position = d.get_mouse_position();
            }

            if self.show_context_menu {
                let menu_items = [
                    "Create >", "Show in Explorer", "Open", "Delete", "Rename", "Copy Path",
                     "Open Scene Additive", "View in Package Manager",
                     "Import New Asset...", "Import Package >", "Export Package...",
                    "Find References In Scene", "Find References In Project", "Select Dependencies",
                     "Refresh", "Reimport", "Reimport All",
                    "Extract From Prefab", "Update UXML Schema",
                     "Generate Lighting", "Open C# Project",
                    "View in Import Activity Window",
                    "Properties"
                ];

                let max_text_width = menu_items.iter().map(|item| self.font_manager.measure_text(item, 16.0, 1.0).x as i32).max().unwrap_or(0);
                let context_menu_width = (max_text_width as f32 * 1.1).ceil();
                let context_menu_height = (menu_items.len() as f32 * 20.0 * 1.1).ceil();

                let context_menu_rect = Rectangle::new(self.context_menu_position.x, self.context_menu_position.y - context_menu_height, context_menu_width, context_menu_height);
                d.draw_rectangle_rec(context_menu_rect, Color::LIGHTGRAY);

                for (i, item) in menu_items.iter().enumerate() {
                    let item_rect = Rectangle::new(context_menu_rect.x, context_menu_rect.y + i as f32 * 20.0, context_menu_rect.width, 20.0);
                    d.draw_rectangle_lines_ex(item_rect, 1.0, Color::BLACK);
                    self.font_manager.draw_text(d, item, Vector2::new(item_rect.x + 10.0, item_rect.y + 2.0), 16.0, 1.0, Color::BLACK);
                }

                if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) && !context_menu_rect.check_collision_point_rec(d.get_mouse_position()) {
                    self.show_context_menu = false;
                }
            }
        }
    }
}
