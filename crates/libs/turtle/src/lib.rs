pub mod tvk;
pub mod renderer;
use glam::{vec3, Mat4};
pub use renderer::*;
pub mod input_manager;
pub use input_manager::*;
pub mod camera;
pub use camera::*;

use winit::{application::ApplicationHandler, event::WindowEvent, keyboard::KeyCode, window::{CursorGrabMode, Window}};
pub type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Default)]
pub struct TurtleApp {
    pub app_data: Option<AppData>
}

pub struct AppData {
    pub instance_groups: Vec<InstanceGroup>,
    pub renderer: Renderer,
    pub window: Window,
    pub time: std::time::Instant,
    pub camera: Camera,
    pub input_manager: InputManager,
}

impl AppData {
    pub fn new(event_loop: &winit::event_loop::ActiveEventLoop) -> AnyResult<Self> {
        let window_attributes = Window::default_attributes()
            .with_maximized(true);
        let window = event_loop.create_window(window_attributes)?;
        let renderer = Renderer::new(&window)?;
        let mesh = renderer.context.create_mesh_from_cube()?;
        let mut instance_group = InstanceGroup::from(mesh);
        instance_group.create_instance_buffer(&renderer.context)?;
        let count = 1000000;        // how many cubes you want
        let radius = 1000.0;      // radius of sphere
        let spacing = 1.0;      // optional multiplier for cube separation

        for i in 0..count {
            // Compute a normalized position on the sphere
            let phi = std::f32::consts::PI * (3.0 - 5.0_f32.sqrt()); // golden angle
            let y = 1.0 - (i as f32 / (count - 1) as f32) * 2.0;     // y from 1 to -1
            let r = (1.0 - y * y).sqrt();                            // radius at that y
            let theta = phi * i as f32;

            let x = theta.cos() * r;
            let z = theta.sin() * r;

            // Position on the sphere scaled by radius
            let position = glam::vec3(x, y, z) * radius * spacing;

            let forward = position.normalize();
            let up = glam::Vec3::Y;
            let right = up.cross(forward).normalize();
            let adjusted_up = forward.cross(right);

            let rotation = Mat4::from_cols(
                right.extend(0.0),
                adjusted_up.extend(0.0),
                forward.extend(0.0),
                glam::Vec4::W,
            );

            let transform = Mat4::from_translation(position) * rotation;

            let instance_data = tvk::InstanceData {
                model: transform,
                color: vec3(x, y, z)
            };

            instance_group.add_instance(
                instance_data,
                true,
            );
        }
        log::warn!("total instances: {}", instance_group.visible_count);
        instance_group.update_gpu_buffer()?;

        window.set_cursor_visible(false);
        window.set_cursor_grab(CursorGrabMode::Locked).unwrap();

        Ok(Self {
            window,
            renderer,
            time: std::time::Instant::now(),
            input_manager: InputManager::default(),
            camera: Camera::default(),
            instance_groups: vec![instance_group]
        })
    }
}

impl TurtleApp {
    pub fn new() -> Self {
        Self {
            app_data: None
        }
    }
}

impl ApplicationHandler for TurtleApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.app_data.is_none() {
            self.app_data = Some(AppData::new(event_loop).unwrap());
        }

        self.app_data.as_ref().unwrap().window.request_redraw();
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        if let Some(app_data) = &mut self.app_data {
            app_data.input_manager.handle_window_event(&event);
        }
        match event {
            WindowEvent::CloseRequested => {
                if let Some(app_data) = &self.app_data {
                    app_data.renderer.context.logical_device.device_wait_idle().unwrap();
                    app_data.renderer.reset_command_buffers().unwrap();
                }
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if let Some(app_data) = &mut self.app_data {
                    if app_data.renderer.render(&app_data.camera, &app_data.instance_groups).unwrap() {
                        app_data.renderer.recreate_swapchain(&app_data.window).unwrap();
                    }
                    app_data.window.request_redraw();
                }
            },
            _ => ()
        }
    }
    
    fn device_event(
            &mut self,
            _event_loop: &winit::event_loop::ActiveEventLoop,
            _device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        ) {
        if let Some(app_data) = &mut self.app_data {
            app_data.input_manager.handle_device_event(&event);
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(app_data) = &mut self.app_data {
            app_data.camera.update(&app_data.renderer.swapchain, &app_data.input_manager);
            
            if app_data.input_manager.keyboard().just_pressed(KeyCode::Escape) {
                app_data.window.set_cursor_visible(true);
                app_data.window.set_cursor_grab(CursorGrabMode::None).unwrap();
            }
            
            app_data.input_manager.update();
        }
    }
}