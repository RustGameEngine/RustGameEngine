use raylib::prelude::*;

pub struct Renderer {
    pub camera: Camera3D,
}

impl Renderer {
    pub fn new() -> Self {
        let camera = Camera3D::perspective(
            Vector3::new(0.0, 10.0, 10.0),
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            45.0,
        );

        Renderer { camera }
    }

    pub fn render(&mut self, d: &mut RaylibDrawHandle) {
        let mut d3d = d.begin_mode3D(self.camera);

        // Draw a simple 3D cube
        d3d.draw_cube(Vector3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0, Color::RED);
        d3d.draw_cube_wires(Vector3::new(0.0, 0.0, 0.0), 2.0, 2.0, 2.0, Color::MAROON);

        // Draw grid
        d3d.draw_grid(10, 1.0);
    }

    pub fn update_camera(&mut self, rl: &mut RaylibHandle) {
        rl.update_camera(&mut self.camera, CameraMode::CAMERA_FIRST_PERSON);
    }

    pub fn render_middle_panel(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::RAYWHITE); // Clear the background with a specific color

        let mut d3d = d.begin_mode3D(self.camera);

        // Draw a 3D object in the middle panel
        d3d.draw_sphere(Vector3::new(0.0, 0.0, 0.0), 1.0, Color::BLUE);
        d3d.draw_sphere_wires(Vector3::new(0.0, 0.0, 0.0), 1.0, 16, 16, Color::DARKBLUE);

        // Draw grid
        d3d.draw_grid(10, 1.0);
    }
}
