use log::debug;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod gl;
use gl::Renderer;

struct App<'window> {
    renderer: Renderer<'window>,
}

impl<'window> App<'window> {
    fn new(renderer: Renderer<'window>) -> Self {
        Self { renderer }
    }
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                debug!("{:?}", event);
            }
            WindowEvent::CursorMoved { position, .. } => {
                debug!("{:?}", position);
            }
            WindowEvent::MouseWheel { delta: _, .. } => {}
            WindowEvent::RedrawRequested => self.renderer.draw(),
            _ => (),
        }
    }
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::max())
        .init();

    let event_loop = EventLoop::new().expect("event loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    let window = event_loop
        .create_window(Window::default_attributes().with_visible(true))
        .expect("winit window");
    let renderer = Renderer::new(&window);
    let mut app = App::new(renderer);

    event_loop
        .run_app(&mut app)
        .expect("event loop dirty shutdown");
}
