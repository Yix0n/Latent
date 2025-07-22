use winit::keyboard::KeyCode;
use winit::keyboard::KeyCode::{KeyH, KeyW};
use crate::app::AppContext;
use crate::engine::{
    renderer::renderer::Renderer,
    math::vector2::Vector2,
};
use crate::engine::events::keyboard::ButtonState::{InputManager, Key};
use crate::engine::renderer::colors::Colors;

pub fn draw_scene(
    renderer: &mut Renderer,
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    input_manager: &InputManager
) {
    renderer.begin_frame();
    
    // Drawing Logic goes here
    
    if input_manager.is_held(Key::Code(KeyW)) {
        println!("W held");
        renderer.draw_rectangle(Vector2::new(100.0, 100.0), 200.0, 150.0, Colors::Red)
    }
    
    if input_manager.is_pressed(Key::Code(KeyW)) {
        println!("W pressed");
    }
    
    if input_manager.is_released(Key::Code(KeyW)) {
        println!("W released");
    }
    
    renderer.end_frame(encoder, view);
}