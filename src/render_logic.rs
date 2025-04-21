use crate::engine::{
    renderer::renderer::Renderer,
    math::vector2::Vector2,
};

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
        [0f32, 1f32, 1f32, 1f32]
    )
}