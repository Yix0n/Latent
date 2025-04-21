use crate::engine::{
    renderer::renderer::Renderer,
    math::vector2::Vector2,
};
use crate::engine::renderer::colors::Colors;

pub fn draw_scene(
    renderer: &Renderer,
    encoder: &mut wgpu::CommandEncoder,
    view: &wgpu::TextureView
) {
    // Drawing Logic goes here
    
    renderer.draw_circle(
        encoder,
        view,
        Vector2::new(0f32, 0f32),
        0.55f32,
        10,
        Colors::Yellow
    )
}