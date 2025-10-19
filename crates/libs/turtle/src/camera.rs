use glam::{Mat4, Vec3};
use crate::*;
pub struct Camera {
    pub position: Vec3,
    pub projection: Mat4,
    pub fov: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 0.0, -5.0),
            projection: glam::Mat4::perspective_infinite_rh(f32::to_radians(45.0), 1.0, 0.1),
            fov: 45.0,
            yaw: 180.0,
            pitch: 0.0,
        }
    }
}

impl Camera {
    pub fn update(&mut self, swapchain: &tvk::Swapchain, input_manager: &InputManager) {
        let aspect_ratio = swapchain.extent.width as f32 / swapchain.extent.height as f32;
        self.projection = glam::Mat4::perspective_infinite_rh(f32::to_radians(self.fov), aspect_ratio, 0.1);
        let sensitivity = 0.25;
        let delta = input_manager.mouse().delta;
        let (dx, dy) = (delta.0 * sensitivity, delta.1 * sensitivity);
        self.yaw += dx;
        self.pitch += dy;
        self.pitch = self.pitch.clamp(-89.0, 89.0);

        let yaw_rad = self.yaw.to_radians();
        let speed = 0.1;
        let keyboard = input_manager.keyboard();
        let forward = Vec3::new(yaw_rad.sin(), 0.0, -yaw_rad.cos()).normalize();
        let right = forward.cross(Vec3::Y).normalize();
        if keyboard.is_pressed(KeyCode::KeyW) {
            self.position += forward * speed;
        }
        if keyboard.is_pressed(KeyCode::KeyS) {
            self.position -= forward * speed;
        }
        if keyboard.is_pressed(KeyCode::KeyA) {
            self.position -= right * speed;
        }
        if keyboard.is_pressed(KeyCode::KeyD) {
            self.position += right * speed;
        }
        if keyboard.is_pressed(KeyCode::Space) {
            self.position -= Vec3::Y * speed;
        }
        if keyboard.is_pressed(KeyCode::ShiftLeft) {
            self.position += Vec3::Y * speed;
        }
        
    }

    pub fn view_matrix(&self) -> Mat4 {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();
        let front = Vec3::new(
            yaw_rad.sin() * pitch_rad.cos(),
            pitch_rad.sin(),
            -yaw_rad.cos() * pitch_rad.cos()
        ).normalize();
        Mat4::look_to_rh(self.position, front, Vec3::Y)
    }
}