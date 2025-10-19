pub mod tvk;
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
    pub renderer: tvk::Renderer,
    pub window: Window,
    pub time: std::time::Instant,
    pub camera: Camera,
    pub input_manager: InputManager,
}

impl AppData {
    pub fn new(event_loop: &winit::event_loop::ActiveEventLoop) -> AnyResult<Self> {
        let window = event_loop.create_window(Window::default_attributes())?;
        let mut renderer = tvk::Renderer::new(&window)?;
        let meshes = vec![renderer.context.create_mesh_from_vertices(tvk::CUBE_VERTICES.into(), tvk::CUBE_INDICES.into())?];
        renderer.meshes = meshes;

        window.set_cursor_visible(false);
        window.set_cursor_grab(CursorGrabMode::Locked).unwrap();

        Ok(Self {
            window,
            renderer,
            time: std::time::Instant::now(),
            input_manager: InputManager::default(),
            camera: Camera::default(),
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
                    if app_data.renderer.render(app_data.time, &app_data.camera).unwrap() {
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