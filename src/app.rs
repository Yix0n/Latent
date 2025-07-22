use std::sync::Arc;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, WindowEvent, KeyEvent, StartCause};
use winit::event::WindowEvent::KeyboardInput;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::engine::renderer::renderer::Renderer;
use crate::render_logic::draw_scene;
use crate::engine::events::keyboard::ButtonState::InputManager;

pub struct State<'a> {
    surface: wgpu::Surface<'a>,
    config: wgpu::SurfaceConfiguration,
    renderer: Renderer,
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: Default::default(),
            required_limits: Default::default(),
        }, None).await.unwrap();

        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let size = window.inner_size();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        window.set_title("Latent");

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shape Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("engine/renderer/shaders/shader.wgsl").into()),
        });

        let size = window.inner_size();

        let renderer = Renderer::new(device, queue, shader, format, size.width, size.height);

        Self { surface, config, renderer }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width == 0 || new_size.height == 0 {
            return;
        }
        
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.renderer.device, &self.config);
        self.renderer.surface_config.width = new_size.width;
        self.renderer.surface_config.height = new_size.height;
    }

    pub fn draw(&self) {

    }
}

pub struct App<'a>{
    context: AppContext<'a>,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self {
            context: AppContext::new(),
        }
    }
}

impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("App resumed");
        if self.context.window.is_none() {
            let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
            self.context.window = Some(window.clone());

            let state = pollster::block_on(State::new(window.clone()));
            self.context.state = Some(state);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if id != self.context.window.as_ref().unwrap().id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("Close requested");
                event_loop.exit()
            },
            WindowEvent::Resized(physical_size) => {
                println!("Resize requested");
                self.context.state.as_mut().unwrap().resize(physical_size);
            },
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let delta = now.duration_since(self.context.last_frame_time);
                self.context.last_frame_time = now;
                let delta_seconds = delta.as_secs_f32();

                // Update Draw, Inputs etc
                self.context.update(delta_seconds);
                self.context.draw();
                
                self.context.window.as_mut().unwrap().request_redraw();
            },
            KeyboardInput {
                event:
                KeyEvent {
                    physical_key,
                    state,
                    ..
                },
                ..
            } => {
                self.context.input_manager.handle_key(physical_key, state);
            }

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

pub struct AppContext<'a> {
    pub window: Option<Arc<Window>>,
    pub state: Option<State<'a>>,
    pub input_manager: InputManager,

    last_frame_time: Instant,
}

impl <'a> AppContext<'a> {
    pub fn new() -> Self{
        Self {
            window: None,
            state: None,
            input_manager: InputManager::default(),
            last_frame_time: Instant::now(),
        }
    }

    pub fn initalize(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
            self.window = Some(window.clone());

            let state = pollster::block_on(State::new(window.clone()));
            self.state = Some(state);
        }
    }

    pub fn draw(&mut self) {
        if let Some(state) = &mut self.state {
            let output = match state.surface.get_current_texture() {
                Ok(tex) => tex,
                Err(_) => return,
            };

            let view = output.texture.create_view(&Default::default());

            let mut encoder = state.renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

            draw_scene(&state.renderer, &mut encoder, &view, &self.input_manager);

            state.renderer.queue.submit(Some(encoder.finish()));
            output.present();
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.input_manager.update();
    }
}