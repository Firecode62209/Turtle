pub mod tvk;
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};
pub type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Default)]
pub struct TurtleApp {
    pub app_data: Option<AppData>
}

pub struct AppData {
    pub renderer: tvk::Renderer,
    pub window: Window,
    pub time: std::time::Instant
}

impl AppData {
    pub fn new(event_loop: &winit::event_loop::ActiveEventLoop) -> AnyResult<Self> {
        let window = event_loop.create_window(Window::default_attributes())?;
        let mut renderer = tvk::Renderer::new(&window)?;
        let meshes = vec![renderer.context.create_mesh_from_vertices(tvk::CUBE_VERTICES.into(), tvk::CUBE_INDICES.into())?];
        renderer.meshes = meshes;

        Ok(Self {
            window,
            renderer,
            time: std::time::Instant::now()        
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
                    if app_data.renderer.render(app_data.time).unwrap() {
                        app_data.renderer.recreate_swapchain(&app_data.window).unwrap();
                    }
                    app_data.window.request_redraw();
                }
            },
            _ => ()
        }
    }
}