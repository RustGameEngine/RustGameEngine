
pub mod camera_controls {
    use crate::camera::Camera;

    pub fn pan_camera(camera: &mut Camera, delta_x: f32, delta_y: f32) {
        camera.position[0] += delta_x;
        camera.position[1] += delta_y;
    }

    pub fn zoom_camera(camera: &mut Camera, zoom_factor: f32) {
        camera.zoom *= zoom_factor;
    }
}
