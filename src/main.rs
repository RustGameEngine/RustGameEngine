mod ui;
mod game_object;
mod camera;
mod cube;
mod my_app;

use crate::my_app::MyApp;
use eframe::NativeOptions;
use image::{DynamicImage, GenericImageView};
fn main() {
    let options = NativeOptions {
        icon_data: Some(load_window_icon("assets/rustgameengine.png")),
        ..Default::default()
    };
    eframe::run_native(
        "RustGameEngine",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
fn load_window_icon(path: &str) -> eframe::IconData {
    // Load image as dynamic image
    let image = image::open(path).expect("Failed to load window icon");
    let rgba_image = image.to_rgba8();
    let (width, height) = image.dimensions();

    eframe::IconData {
        rgba: rgba_image.into_raw(),
        width: width,
        height: height,
    }
}
