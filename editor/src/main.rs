use log::debug;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod gl;
use gl::Renderer;

struct App {
    renderer: Option<Renderer>,
}

impl App {
    fn new() -> Self {
        Self { renderer: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .expect("winit window");
        self.renderer = Some(Renderer::new(window));
    }

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
            WindowEvent::Resized(size) => {
                if let Some(ref renderer) = self.renderer {
                    renderer.resize(size);
                }
            }
            WindowEvent::RedrawRequested => self.renderer.as_ref().expect("winit").draw(),
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
    let mut app = App::new();

    event_loop
        .run_app(&mut app)
        .expect("event loop dirty shutdown");
}
