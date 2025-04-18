use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct State<'a> {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'a>
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(Arc::clone(&window)).unwrap();

        Self {
            instance, surface
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        println!("Resizing to {:?}", new_size);
    }

    pub fn draw(&self) {
        println!("Drawing");
    }
}

#[derive(Default)]
pub struct App<'a>{
    window: Option<Arc<Window>>,
    state: Option<State<'a>>,
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("App resumed");
        if self.window.is_none() {
            let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
            self.window = Some(window.clone());

            let state = pollster::block_on(State::new(window.clone()));
            self.state = Some(state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if id != self.window.as_ref().unwrap().id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested");
                event_loop.exit()
            },
            WindowEvent::Resized(physical_size) => {
                println!("Resize requested");
                self.state.as_mut().unwrap().resize(physical_size);
            },
            WindowEvent::RedrawRequested => {
                println!("Redraw requested");
                self.state.as_ref().unwrap().draw();
            },
            _ => {},
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        println!("App suspended");
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        println!("App exiting");
    }
}
