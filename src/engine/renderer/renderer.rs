use wgpu::util::DeviceExt;
use crate::engine::math::vector2::Vector2;
use bytemuck::{Pod, Zeroable};
use wgpu::StoreOp;
use crate::engine::renderer::colors::Colors;

pub struct Renderer{
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub pipeline: wgpu::RenderPipeline,
    pub surface_config: wgpu::SurfaceConfiguration,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout{
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as u64,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                }
            ]
        }
    }
}

impl Renderer {
    pub fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        shader: wgpu::ShaderModule,
        format: wgpu::TextureFormat,
        width: u32,
        height: u32,
    ) -> Self {
        let surface_config = wgpu::SurfaceConfiguration{
            usage:wgpu::TextureUsages::RENDER_ATTACHMENT,
            format, width, height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 0,
            alpha_mode: Default::default(),
            view_formats: vec![wgpu::TextureFormat::Rgba32Float],
        };

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Shape Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self { device, queue, pipeline, surface_config }
    }
    pub fn draw_rectangle(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        pos: Vector2,
        width: f32,
        height: f32,
        color: Colors,
    ) {
        let color = color.as_f32();

        let window_size = self.get_window_size();

        let vertices = [
            Vertex { position: Vector2::new(pos.x, pos.y).to_ndc(window_size), color },
            Vertex { position: Vector2::new(pos.x + width, pos.y).to_ndc(window_size), color },
            Vertex { position: Vector2::new(pos.x + width, pos.y + height).to_ndc(window_size), color },
            Vertex { position: Vector2::new(pos.x, pos.y).to_ndc(window_size), color },
            Vertex { position: Vector2::new(pos.x + width, pos.y + height).to_ndc(window_size), color },
            Vertex { position: Vector2::new(pos.x, pos.y + height).to_ndc(window_size), color },
        ];

        self.render_vertices(encoder, view, &vertices);
    }


    pub fn draw_triangle(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        a: Vector2,
        b: Vector2,
        c: Vector2,
        color: Colors
    ){
        let color = color.as_f32();

        let window_size = self.get_window_size();

        let vertices = [
            Vertex { position: Vector2::new(a.x, a.y).to_ndc(window_size), color },
            Vertex { position: Vector2::new(b.x, b.y).to_ndc(window_size), color },
            Vertex { position: Vector2::new(c.x, c.y).to_ndc(window_size), color },
        ];

        self.render_vertices(encoder, view, &vertices);
    }

    pub fn draw_circle(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        center: Vector2,
        radius: f32,
        segments: usize,
        color: Colors
    ) {
        let mut vertices = Vec::with_capacity(segments * 3);

        let window_size = self.get_window_size();
        
        for i in 0..segments {
            let theta1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let theta2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;

            let p1 = Vector2 {
                x: center.x + radius * theta1.cos(),
                y: center.y + radius * theta1.sin(),
            };

            let p2 = Vector2 {
                x: center.x + radius * theta2.cos(),
                y: center.y + radius * theta2.sin(),
            };

            let color = color.as_f32();

            vertices.push(Vertex { position: Vector2::new(center.x, center.y).to_ndc(window_size), color });
            vertices.push(Vertex { position: Vector2::new(p1.x, p1.y).to_ndc(window_size), color });
            vertices.push(Vertex { position: Vector2::new(p2.x, p2.y).to_ndc(window_size), color });
        }

        self.render_vertices(encoder, view, &vertices);
    }

    fn render_vertices(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
        vertices: &[Vertex],
    ) {
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Render Shape"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Shape"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: StoreOp::Store,
                }
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..vertices.len() as u32, 0..1);
    }

    fn get_window_size(&self) -> Vector2 {
        Vector2::new(self.surface_config.width as f32, self.surface_config.height as f32)
    }
}