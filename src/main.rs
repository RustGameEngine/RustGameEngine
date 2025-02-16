mod ui;
mod panels;
mod font;
mod intro;

use raylib::prelude::*;
use raylib::ffi::ConfigFlags;
use std::sync::{Arc, Mutex};
use ui::UI;
use panels::Panels;
use font::FontManager;
use intro::Intro;

const DEFAULT_BOTTOM_PANEL_HEIGHT_RATIO: f32 = 0.2;
const DEFAULT_RIGHT_PANEL_WIDTH_RATIO: f32 = 0.2;
const DEFAULT_LEFT_PANEL_WIDTH_RATIO: f32 = 0.2;
const MENU_BAR_HEIGHT_RATIO: f32 = 0.03;

fn main() {
    show_editor();
}

fn show_editor(){

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("RustGameEngine")
        //.vsync()
        .resizable()
        .build();

    unsafe {
        raylib::ffi::SetWindowState(ConfigFlags::FLAG_WINDOW_MAXIMIZED as u32);
            // Disable ESC to quit
    raylib::ffi::SetExitKey(0);
    }
    let screen_height = rl.get_screen_height();
    let screen_width = rl.get_screen_width();

    let mut panels = Panels::new(
        (screen_height as f32 * DEFAULT_BOTTOM_PANEL_HEIGHT_RATIO) as i32,
        (screen_width as f32 * DEFAULT_RIGHT_PANEL_WIDTH_RATIO) as i32,
        (screen_width as f32 * DEFAULT_LEFT_PANEL_WIDTH_RATIO) as i32,
    );

    let menu_bar_height = (screen_height as f32 * MENU_BAR_HEIGHT_RATIO) as i32;

    let font_manager = FontManager::new(&mut rl, &thread).expect("Failed to load font");
    let intro = Intro::new(&mut rl, &thread); // Initialize Intro struct
    let mut ui = UI::new(font_manager); // Pass Intro to UI::new

    let console_log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let console_log_clone = Arc::clone(&console_log);

    // Simulate logging
    std::thread::spawn(move || {
        let mut log = console_log_clone.lock().unwrap();
        log.push("Hello, world!".to_string());
    });

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();

        ui.draw(&mut d, screen_width, screen_height, panels.bottom_panel_height, panels.right_panel_width, panels.left_panel_width, menu_bar_height, &console_log);

        panels.handle_resizing(&mut d, screen_width, screen_height, menu_bar_height);
    }
}
