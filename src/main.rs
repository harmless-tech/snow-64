mod logging;
mod texture;
mod graphics;

use anyhow::*;
use configparser::ini;
use futures::executor::block_on;
use image::GenericImageView;
use log::{debug, error, info, trace, warn};
use winit::{
    dpi,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window,
    window::{WindowBuilder},
};
use crate::graphics::WGPUState;
use std::time::Instant;

// Planning
// Render textures on top of one another.
// Scale camera to the position on the texture.
// Allow alpha textures to be stacked on top of one another.

const DISPLAY_RES: u32 = 256;

#[cfg(debug_assertions)]
const DEBUG_BUILD: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG_BUILD: bool = false;

struct Snow; //TODO

struct SnowConfig {
    display_res: u32,
    dev_debug: bool,
}
impl Default for SnowConfig {
    fn default() -> Self {
        Self {
            display_res: DISPLAY_RES,
            dev_debug: DEBUG_BUILD,
        }
    }
}

pub struct Camera {
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

        graphics::OPENGL_TO_WGPU_MATRIX * proj * view
    }
}

fn main() -> Result<()> {
    let config = load_config()?;
    let mut args = std::env::args();

    let debug = DEBUG_BUILD || args.any(|s| s.eq("--debug")) || config.dev_debug;

    let _logging = logging::setup_log(debug)?;
    info!("Logging Check!");
    info!("Logging Level Info: TRUE");
    warn!("Logging Level Warn: TRUE");
    error!("Logging Level Error: TRUE");
    debug!("Logging Level Debug: TRUE");
    trace!("Logging Level Trace: TRUE");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_min_inner_size(dpi::Size::Physical(dpi::PhysicalSize::new(
            config.display_res,
            config.display_res,
        )))
        .with_inner_size(dpi::Size::Physical(dpi::PhysicalSize::new(
            config.display_res,
            config.display_res,
        )))
        .with_resizable(false)
        .with_window_icon(Some(load_window_icon()?))
        .with_title("Snow64 - alpha build")
        .build(&event_loop)
        .unwrap();

    let mut state = block_on(WGPUState::new(&window));

    //
    let mut fps_counter: u64 = 0;
    let mut last_fps_time = Instant::now();

    const FIXED_UPDATE_TIME: f64 = 1.0 / 60.0; //TODO Why is this 48 fps?
    let mut fixed_counter: u64 = 0;
    let mut last_fixed_time = Instant::now();
    //

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
            fps_counter += 1;

            if Instant::now().duration_since(last_fixed_time).as_secs_f64() >= FIXED_UPDATE_TIME {
                last_fixed_time = Instant::now();
                fixed_counter += 1;
                //TODO
            }

            if Instant::now().duration_since(last_fps_time).as_nanos() >= 1_000_000_000 {
                last_fps_time = Instant::now();
                debug!("FPS: {}", fps_counter);
                debug!("Fixed: {}", fixed_counter);
                fps_counter = 0;
                fixed_counter = 0;
            }

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
}

fn load_config() -> Result<SnowConfig> {
    let mut config = SnowConfig::default();

    let file = std::fs::read_to_string("./snow64-data/config.ini").unwrap_or("".to_string());
    if !file.is_empty() {
        let mut parser = ini::Ini::new();
        parser.read(file).map_err(|e| Error::msg(e))?;

        match parser
            .getuint("display", "res")
            .map_err(|e| Error::msg(e))?
        {
            None => {}
            Some(val) => {
                if val as u32 >= DISPLAY_RES {
                    config.display_res = val as u32;
                }
            }
        }
        match parser.getbool("dev", "debug").map_err(|e| Error::msg(e))? {
            None => {}
            Some(val) => config.dev_debug = val,
        }
    }

    Ok(config)
}

fn load_window_icon() -> Result<window::Icon> {
    let bytes = include_bytes!("./assets/icons/icon-512.oxi.png");
    let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?;
    let rgba = img.as_rgba8().unwrap();
    let dimensions = img.dimensions();

    window::Icon::from_rgba(rgba.clone().into_vec(), dimensions.0, dimensions.1)
        .context("Failed to create window icon!")
}
