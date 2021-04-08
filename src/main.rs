mod logging;
mod texture;

use anyhow::*;
use cgmath::prelude::*;
use futures::executor::block_on;
use image::GenericImageView;
use log::{debug, error, info, trace, warn};
use wgpu::{util::DeviceExt};
use winit::{
    dpi,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window,
    window::{Window, WindowBuilder},
};

// Planning
// Render textures on top of one another.
// Scale camera to the position on the texture.
// Allow alpha textures to be stacked on top of one another.

const WIDTH: u32 = 1024; //TODO Set back to 256.
const HEIGHT: u32 = 1024; //TODO Set back to 256.

#[cfg(debug_assertions)]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex_coords: [0.0, 0.0],
    }, // A
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex_coords: [0.0, 1.0],
    }, // B
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex_coords: [1.0, 1.0],
    }, // C
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 0.0],
    }, // D
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
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
                    format: wgpu::VertexFormat::Float3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float2,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
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
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

struct Camera {
    eye: cgmath::Point3<f32>,
    target: cgmath::Point3<f32>,
    up: cgmath::Vector3<f32>,
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}
impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

struct WState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
    clear_color: wgpu::Color,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    num_vertices: u32,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_texture: texture::Texture,
    camera: Camera,
    camera_controller: CameraController,
    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    depth_texture: texture::Texture,
}
impl WState {
    async fn new(window: &Window) -> Self {
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

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::default(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo, //TODO Change?
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let clear_color = wgpu::Color::BLACK;

        let diffuse_bytes = include_bytes!("./assets/icons/icon-256.png");
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "icon-256.png").unwrap();

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
                        count: None,
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
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Diffuse Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
        });

        let camera = Camera {
            eye: (0.0, 0.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            aspect: 1.0, /*sc_desc.width as f32 / sc_desc.height as f32*/
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let camera_controller = CameraController::new(0.5);

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
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
                clamp_depth: false,
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
            diffuse_texture,
            camera,
            camera_controller,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            depth_texture,
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);

        self.depth_texture =
            texture::Texture::create_depth_texture(&self.device, &self.sc_desc, "Depth Texture");
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event);
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
            _ => false,
        }
    }

    fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.uniforms.update_view_proj(&self.camera);
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }

    fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture.view,
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

//TODO Config.ini
fn main() -> Result<()> {
    let mut args = std::env::args();
    let debug = DEBUG || args.any(|s| s.eq("--debug"));

    let _logging = logging::setup_log(debug)?;
    info!("Logging Check!");
    info!("Logging Level Info: TRUE");
    warn!("Logging Level Warn: TRUE");
    error!("Logging Level Error: TRUE");
    debug!("Logging Level Debug: TRUE");
    trace!("Logging Level Trace: TRUE");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_min_inner_size(dpi::Size::Physical(dpi::PhysicalSize::new(WIDTH, HEIGHT)))
        .with_inner_size(dpi::Size::Physical(dpi::PhysicalSize::new(WIDTH, HEIGHT)))
        .with_window_icon(Some(create_window_icon()?))
        .with_title("Snow64 - alpha build")
        .build(&event_loop)
        .unwrap();

    let mut state = block_on(WState::new(&window));

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !state.input(event) {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    WindowEvent::Resized(size) => {
                        //TODO This is fine for now. Should be greatly improved in future.

                        if size.width == size.height {
                            state.resize(*size);
                        }
                        else {
                            let max = size.width.max(size.height);
                            window.set_inner_size(dpi::Size::Physical(dpi::PhysicalSize::new(
                                max, max,
                            )));
                        }
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size)
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(_) => {
            state.update();
            match state.render() {
                Ok(_) => {}
                Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => error!("{:?}", e),
            }
        }
        Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    });

    //Ok(())
}

fn create_window_icon() -> Result<window::Icon> {
    let bytes = include_bytes!("./assets/icons/icon-512.png");
    let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?;
    let rgba = img.as_rgba8().unwrap();
    let dimensions = img.dimensions();
    window::Icon::from_rgba(rgba.clone().into_vec(), dimensions.0, dimensions.1)
        .context("Failed to create window icon!")
}

// FROM: https://sotrh.github.io/learn-wgpu/beginner/tutorial6-uniforms/#a-controller-for-our-camera
struct CameraController {
    speed: f32,
    is_up_pressed: bool,
    is_down_pressed: bool,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}
impl CameraController {
    fn new(speed: f32) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
        }
    }

    fn process_events(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state,
                        virtual_keycode: Some(keycode),
                        ..
                    },
                ..
            } => {
                let is_pressed = *state == ElementState::Pressed;
                match keycode {
                    VirtualKeyCode::Space => {
                        self.is_up_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::LShift => {
                        self.is_down_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::W | VirtualKeyCode::Up => {
                        self.is_forward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::A | VirtualKeyCode::Left => {
                        self.is_left_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::S | VirtualKeyCode::Down => {
                        self.is_backward_pressed = is_pressed;
                        true
                    }
                    VirtualKeyCode::D | VirtualKeyCode::Right => {
                        self.is_right_pressed = is_pressed;
                        true
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        // Prevents glitching when camera gets too close to the
        // center of the scene.
        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        // Redo radius calc in case the up/ down is pressed.
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            // Rescale the distance between the target and eye so
            // that it doesn't change. The eye therefore still
            // lies on the circle made by the target and eye.
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}
