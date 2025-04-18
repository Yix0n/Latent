use wgpu::{Device, Queue, RenderPipeline, TextureView, SurfaceConfiguration, CommandEncoder, util::DeviceExt, BindGroupLayout, BindGroup, Texture, ShaderStages, BindingType, TextureViewDimension, TextureSampleType, PipelineLayoutDescriptor, RenderPipelineDescriptor, VertexState, FragmentState, PrimitiveState, MultisampleState, Extent3d, TextureDimension, TextureFormat, TextureUsages, ImageCopyTexture, Origin3d, TextureAspect, ImageDataLayout, BindGroupDescriptor, BindGroupEntry, BindingResource, RenderPassDescriptor, RenderPassColorAttachment, Operations, LoadOp, StoreOp, TextureDescriptor, TextureViewDescriptor, SamplerDescriptor};

use std::{
    collections::HashMap,
    path::Path,
};

use image::GenericImageView;
use crate::engine::math::vector2::Vector2;

pub struct SpriteRenderer {
    pipeline: RenderPipeline,
    texture_bind_group_layout: BindGroupLayout,
    textures: HashMap<String, (Texture, BindGroup)>,
}

impl SpriteRenderer {
    pub fn new(device: &Device, config: &SurfaceConfiguration) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Sprite Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shaders/sprite_shader.wgsl").into()),
        });
        
        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Texture Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture { 
                    multisampled: false,
                    view_dimension: TextureViewDimension::D2,
                    sample_type: TextureSampleType::Float { filterable: true },
                    
                },
                count: None,
            }]
        });
        
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Sprite Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });
        
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Sprite Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(config.format.into())]
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
        });
        
        Self {
            pipeline,
            texture_bind_group_layout,
            textures : HashMap::new()
        }
    }
    
    pub fn load_texture(&mut self, device: Device, queue: &Queue, label: &str, path: &Path) {
        let img = image::open(path).unwrap();
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();
        
        let texture_size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        
        let texture = device.create_texture(&TextureDescriptor {
            label: Some(label),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Snorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        queue.write_texture(
            ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        &rgba,
        ImageDataLayout{
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        },
        texture_size
        );
        
        let view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor::default());
        
        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0, 
                    resource: BindingResource::TextureView(&view)
                },
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Sampler(&sampler)
                }
            ],
            label: Some("Texture Bind Group")
        });
        
        self.textures.insert(label.to_string(), (texture, bind_group));
    }
    
    pub fn render_sprite_local(&self, encoder: &mut CommandEncoder, view: &TextureView, label: &str, position: Vector2) {
        if let Some((_, bind_group)) = self.textures.get(label) {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor{
                label: Some("Sprite Render Pass Local"),
                color_attachments: &[Some(RenderPassColorAttachment{
                    view,
                    resolve_target: None,
                    ops: Operations{
                        load: LoadOp::Load,
                        store: StoreOp::Store
                    }
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.draw(0..3, 0..1); // Placeholder quad
        }
    }
    
    pub fn render_text(&self, text: &str, position: Vector2) {
        // Placeholder: Integrate font for rendering
        println!("render_text: {}", text);
    }
}