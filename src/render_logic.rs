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
    renderer: &Renderer,
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    input_manager: &InputManager
) {
    // Drawing Logic goes here

    renderer.draw_circle(
        encoder,
        view,
        Vector2::new(500f32, 200f32),
        10f32,
        10,
        Colors::Yellow
    );

    renderer.draw_triangle(
        encoder,
        view,
        Vector2::new(400f32, 200f32),
        Vector2::new(400f32, 100f32),
        Vector2::new(200f32, 100f32),
        Colors::Yellow
    );
    
    if input_manager.is_held(Key::Code(KeyW)) {
        println!("W held");
    }
    
    if input_manager.is_pressed(Key::Code(KeyW)) {
        println!("W pressed");
    }
    
    if input_manager.is_released(Key::Code(KeyW)) {
        println!("W released");
    }
}