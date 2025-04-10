mod ui;
mod game_object;
mod camera;
mod cube;
mod my_app;

use crate::my_app::MyApp;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RustGameEngine",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
