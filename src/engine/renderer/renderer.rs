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

    vertex_buffer: wgpu::Buffer,
    max_vertices: usize,

    vertices: Vec<Vertex>,
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
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 2]>() as u64,
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
        max_vertices: usize,
    ) -> Self {
        let surface_config = wgpu::SurfaceConfiguration{
            usage:wgpu::TextureUsages::RENDER_ATTACHMENT,
            format, width, height,
            present_mode: wgpu::PresentMode::Fifo,
            desired_maximum_frame_latency: 0,
            alpha_mode: Default::default(),
            view_formats: vec![],
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

        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (max_vertices * std::mem::size_of::<Vertex>()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self { device, queue, pipeline, surface_config, vertex_buffer, max_vertices, vertices: Vec::with_capacity(max_vertices) }
    }

    pub fn begin_frame(&mut self) {
        self.vertices.clear();
    }
    pub fn draw_rectangle(&mut self, pos: Vector2, width: f32, height: f32, color: Colors) {
        let window_size = Vector2::new(self.surface_config.width as f32, self.surface_config.height as f32);
        let color = color.as_f32();

        let top_left = pos.to_ndc(window_size);
        let top_right = Vector2::new(pos.x + width, pos.y).to_ndc(window_size);
        let bottom_left = Vector2::new(pos.x, pos.y + height).to_ndc(window_size);
        let bottom_right = Vector2::new(pos.x + width, pos.y + height).to_ndc(window_size);

        // Dwie trójkąty tworzące prostokąt
        self.vertices.extend_from_slice(&[
            Vertex { position: top_left, color },
            Vertex { position: bottom_left, color },
            Vertex { position: top_right, color },

            Vertex { position: top_right, color },
            Vertex { position: bottom_left, color },
            Vertex { position: bottom_right, color },
        ]);
    }

    pub fn draw_triangle(&mut self, a: Vector2, b: Vector2, c: Vector2, color: Colors) {
        let window_size = Vector2::new(self.surface_config.width as f32, self.surface_config.height as f32);
        let color = color.as_f32();

        self.vertices.extend_from_slice(&[
            Vertex { position: a.to_ndc(window_size), color },
            Vertex { position: b.to_ndc(window_size), color },
            Vertex { position: c.to_ndc(window_size), color },
        ]);
    }

    pub fn draw_circle(&mut self, center: Vector2, radius: f32, segments: usize, color: Colors) {
        let window_size = Vector2::new(self.surface_config.width as f32, self.surface_config.height as f32);
        let color = color.as_f32();

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

            self.vertices.extend_from_slice(&[
                Vertex { position: center.to_ndc(window_size), color },
                Vertex { position: p1.to_ndc(window_size), color },
                Vertex { position: p2.to_ndc(window_size), color },
            ]);
        }
    }

    pub fn end_frame(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) {
        assert!(self.vertices.len() <= self.max_vertices, "Przekroczono max_vertices!");

        self.queue.write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(&self.vertices),
        );

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.1,
                        b: 0.1,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertices.len() as u32, 0..1);
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