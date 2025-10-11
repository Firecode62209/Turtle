pub mod tvk;
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};
pub type AnyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Default)]
pub struct TurtleApp {
    pub window: Option<Window>,
    pub renderer: Option<tvk::Renderer>,
    
}

impl TurtleApp {
    pub fn new () -> Self {
        Self {
            renderer: None,
            window: None
        }
    }
}

impl ApplicationHandler for TurtleApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window = event_loop.create_window(Window::default_attributes()).unwrap();
        
            self.renderer = Some(tvk::Renderer::new(&window).unwrap());
            self.window = Some(window);

            self.window.as_ref().unwrap().request_redraw();
        }
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if self.renderer.as_mut().unwrap().render().unwrap() {
                    self.renderer.as_mut().unwrap().recreate_swapchain(self.window.as_ref().unwrap()).unwrap();
                }
                self.window.as_ref().unwrap().request_redraw();
            },
            _ => ()
        }
    }
}