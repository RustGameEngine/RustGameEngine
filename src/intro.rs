use raylib::prelude::*;

pub struct Intro {
    pub logo_texture: Texture2D,
    pub elapsed_time: f32,
}

impl Intro {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        // Adjust the file path to ensure it points to the correct location
        let logo_image = Image::load_image("src/misc/rustgameengine.png").expect("Failed to load logo image");
        let logo_texture = rl.load_texture_from_image(thread, &logo_image).expect("Failed to create texture from logo image");

        Self {
            logo_texture,
            elapsed_time: 0.0,
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle, screen_width: i32, screen_height: i32, dt: f32) {
        self.elapsed_time += dt;
        d.clear_background(Color::BLACK);
        let logo_x = (screen_width / 2) as f32;
        let logo_y = (screen_height / 2) as f32;
        let rotation_angle = self.elapsed_time * 72.0;
        d.draw_texture_ex(&self.logo_texture, Vector2::new(logo_x, logo_y), rotation_angle, 1.0, Color::WHITE);
    }

    pub fn is_finished(&self) -> bool {
        self.elapsed_time > 5.0
    }
}
