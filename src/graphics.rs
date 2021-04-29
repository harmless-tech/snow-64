//TODO Clean!

use crate::{texture, Camera};
use anyhow::*;
use cgmath::prelude::*;
use log::{debug, error, info, trace, warn};
use std::num::NonZeroU32;
use wgpu::util::DeviceExt;
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    }, // A
    Vertex {
        position: [-1.0, -31.0, 0.0],
        tex_coords: [0.0, 1.0],
    }, // B
    Vertex {
        position: [63.0, -31.0, 0.0],
        tex_coords: [1.0, 1.0],
    }, // C
    Vertex {
        position: [63.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    }, // D
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

/**
 * Index:
 * - 0-127 = White Text
 * - 128-255 = Black Text
 * - 256-511 = Tiles
 * - 512-767 = Sprites
 * - 768-... = Pixel Layer
 *
 * Position:
 * - Text - (0-31, 0-31)
 * - Tile - (0-15, 0-15)
 * - Sprite - (-16-272, -16-272)
 * - Pixel - (0-255, 0-255)
 */
/*#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexMap { //TODO !!!
    index: u32,
    position: [i32; 2],
    color: [u32; 4]
}
impl VertexMap {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexIgnore>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Int2,
                    offset: 0,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint4,
                    offset: 0,
                    shader_location: 2,
                },
            ],
        }
    }

    //TODO Safer wrapper over these methods.

    pub fn from_tile(tile: u32) -> Self {
    }

    pub fn from_sprite(sprite: u32) -> Self {
    }

    pub fn from_pixel(x: u32, y: u32, color: [u32; 4]) -> Self {
    }
}*/

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    view_proj: [[f32; 4]; 4],
}
impl Uniforms {
    fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct WGPUState {
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_desc: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub clear_color: wgpu::Color,
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub diffuse_bind_group: wgpu::BindGroup,
    pub diffuse_textures: Vec<texture::Texture>,
    pub camera: Camera, // Move this
    pub uniforms: Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub depth_texture: texture::Texture,

    pub key_mod_state: winit::event::ModifiersState, //TODO
}
impl WGPUState {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        {
            let adapter_info = adapter.get_info();
            info!("Using GPU: {}.", adapter_info.name);
            info!("Using Render API: {:?}.", adapter_info.backend);
        }

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::default() | wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY,
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(), //TODO More safety!
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox, //TODO Allow for vsync mode?
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let clear_color = wgpu::Color::BLACK;

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: NonZeroU32::new(5),
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            filtering: true,
                            comparison: false,
                        },
                        count: None,
                    },
                ],
            });

        //TODO Clean!
        let mut img = image::RgbaImage::from(image::ImageBuffer::new(64, 64));

        for x in 0..64 {
            for y in 0..64 {
                img.put_pixel(x, y, image::Rgba([0, 0, 0, 255]))
            }
        }

        for one in 0..64 {
            for two in vec![0, 64 - 1].iter() {
                img.put_pixel(one, *two, image::Rgba([255, 0, 0, 255]));
                img.put_pixel(*two, one, image::Rgba([255, 0, 0, 255]));
            }
        }

        let diffuse_bytes = &image::DynamicImage::ImageRgba8(img);
        //

        let mut diffuse_textures = Vec::new();
        diffuse_textures.push(texture::Texture::from_image(
            &device,
            &queue,
            diffuse_bytes,
            Some("Layer 0"),
        ));
        diffuse_textures.push(texture::Texture::from_image(
            &device,
            &queue,
            diffuse_bytes,
            Some("Layer 1"),
        ));
        diffuse_textures.push(texture::Texture::from_image(
            &device,
            &queue,
            diffuse_bytes,
            Some("Layer 2"),
        ));
        diffuse_textures.push(texture::Texture::from_image(
            &device,
            &queue,
            diffuse_bytes,
            Some("Layer 3"),
        ));
        diffuse_textures.push(texture::Texture::from_image(
            &device,
            &queue,
            diffuse_bytes,
            Some("Layer 4"),
        ));

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Diffuse Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureViewArray(&[
                        &diffuse_textures.get(0).unwrap().view,
                        &diffuse_textures.get(1).unwrap().view,
                        &diffuse_textures.get(2).unwrap().view,
                        &diffuse_textures.get(3).unwrap().view,
                        &diffuse_textures.get(4).unwrap().view,
                    ]),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture::create_sampler(&device)),
                },
            ],
        });

        let camera = Camera {
            eye: (0.0, 0.0, 1.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: sc_desc.width as f32 / sc_desc.height as f32,
            fovy: 90.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Uniform Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniform Bind Group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let depth_texture =
            texture::Texture::create_depth_texture(&device, &sc_desc, "Depth Texture");

        let vs_module =
            device.create_shader_module(&wgpu::include_spirv!("./assets/shaders/shader.vert.spv"));
        let fs_module =
            device.create_shader_module(&wgpu::include_spirv!("./assets/shaders/shader.frag.spv"));
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&texture_bind_group_layout, &uniform_bind_group_layout],
                push_constant_ranges: &[],
            });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    write_mask: wgpu::ColorWrite::ALL,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING)
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                clamp_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let num_vertices = VERTICES.len() as u32;

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsage::INDEX,
        });
        let num_indices = INDICES.len() as u32;

        let key_mod_state = winit::event::ModifiersState::default();

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            clear_color,
            render_pipeline,
            vertex_buffer,
            num_vertices,
            index_buffer,
            num_indices,
            diffuse_bind_group,
            diffuse_textures,
            camera,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            depth_texture,

            key_mod_state,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);

        self.depth_texture =
            texture::Texture::create_depth_texture(&self.device, &self.sc_desc, "Depth Texture");
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.clear_color = wgpu::Color {
                    r: 1.0,
                    g: position.x / self.size.width as f64,
                    b: position.y / self.size.height as f64,
                    a: 1.0,
                };
                true
            }
            WindowEvent::ModifiersChanged(state) => {
                self.key_mod_state = *state;
                true
            }
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::C),
                    ..
                } => {
                    if self.key_mod_state.ctrl() {
                        debug!("Ctrl-C");
                        return true;
                    }

                    false
                }
                _ => false,
            },
            _ => false,
        }
    }

    pub fn update(&mut self) {
        self.uniforms.update_view_proj(&self.camera);

        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.render_pipeline);

            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }
}