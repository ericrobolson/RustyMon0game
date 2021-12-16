pub use winit::{event::*, event_loop::ControlFlow, window::WindowBuilder};
pub use winit::{event_loop::EventLoop, window::Window};

/// Creates the initial window.
pub fn init_window() -> (EventLoop<()>, Window) {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    (event_loop, window)
}
