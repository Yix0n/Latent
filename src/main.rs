mod app;
mod engine;

use winit::{event_loop::EventLoop};
use winit::event_loop::ControlFlow;
use crate::app::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    if let Err(e) = event_loop.run_app(&mut app) {
        eprintln!("{:?}", e);
    }
}
