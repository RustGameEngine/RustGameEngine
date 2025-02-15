use raylib::prelude::*;
use raylib::ffi::{MeasureTextEx, Font as FfiFont};
use std::ffi::CString;

pub struct FontManager {
    pub font: Font,
}

impl FontManager {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Result<Self, String> {
        let font_path = "src/fonts/Roboto-Bold.ttf";
        match rl.load_font(thread, font_path) {
            Ok(font) => Ok(FontManager { font }),
            Err(e) => {
                eprintln!("Failed to load font: {}", e);
                Err(format!("Failed to load font: {}", e))
            },
        }
    }

    pub fn draw_text(&self, d: &mut RaylibDrawHandle, text: &str, position: Vector2, font_size: f32, spacing: f32, color: Color) {
        // Enable anti-aliasing
        d.draw_text_ex(&self.font, text, position, font_size, spacing, color);
    }
    
    pub fn measure_text(&self, text: &str, font_size: f32, spacing: f32) -> Vector2 {
        let c_text = CString::new(text).expect("CString conversion failed"); // Ensure null-terminated string
        let ffi_font: FfiFont = unsafe { std::mem::transmute_copy(&self.font) };
        let ffi_vector = unsafe { 
            MeasureTextEx(ffi_font, c_text.as_ptr(), font_size, spacing) 
        };
        Vector2::new(ffi_vector.x, ffi_vector.y)
    }
    
}
