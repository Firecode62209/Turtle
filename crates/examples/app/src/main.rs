use turtle::{AnyResult, TurtleApp};
use winit::event_loop::{ControlFlow, EventLoop};


fn main() -> AnyResult<()> {
    pretty_env_logger::init();
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = TurtleApp::default();
    event_loop.run_app(&mut app).unwrap();
    Ok(())
}