mod config;
mod logging;

use anyhow::*;
use image::GenericImageView;
use log::{debug, error, info, trace, warn};
use std::{thread, time::Instant};
use winit::{
    dpi,
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window,
    window::WindowBuilder,
};

// Planning
// Render textures on top of one another.
// Scale camera to the position on the texture.
// Allow alpha textures to be stacked on top of one another.

/* TODO Maybe?
 * Uploaded tile sheets and sprite sheets to the gpu and have a shader do the rendering.
 * The pixel layer rendering will remain the same? Or maybe upload an array of colors and have the
 *     GPU do the math?
 *
 * TODO Maybe?
 * Have a big buffer in the program.
 * Upload a small part of it to the gpu.
 */

const FIXED_LOOP_TIME: u64 = 16666667 - 2800000;

struct Snow64; //TODO

fn main() -> Result<()> {
    let config = config::load_config()?;
    let _logging = logging::setup_log(config.dev_debug)?;

    let event_loop = EventLoop::new();
    //let mut input = WinitInputHelper::new(); TODO: Use this?
    let window = WindowBuilder::new()
        .with_min_inner_size(dpi::Size::Physical(dpi::PhysicalSize::new(
            config.display_res,
            config.display_res,
        )))
        .with_inner_size(dpi::Size::Physical(dpi::PhysicalSize::new(
            config.display_res,
            config.display_res,
        )))
        .with_resizable(true)
        .with_window_icon(Some(load_window_icon()?))
        .with_title("Snow64 - alpha build")
        .build(&event_loop)
        .unwrap();
    let main_window_id = window.id();

    // Pixels setup

    // For DBG, remove later
    let mut fps_counter: u64 = 0;
    let mut last_time = Instant::now();
    //

    // Loop
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_nanos(FIXED_LOOP_TIME));
        window.request_redraw();
    });

    //TODO: If statements might look nicer?
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                // For DBG, remove later
                fps_counter += 1;
                if last_time.elapsed().as_secs() >= 1 {
                    last_time = Instant::now();
                    info!("FPS: {}", fps_counter);
                    fps_counter = 0;
                }
                //

                //TODO: Render update stuff.
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == main_window_id => {
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
                        //TODO: Pixels resize.
                    }
                    _ => {}
                }
                //TODO: There should be a pass to other input handlers here.
            }
            _ => {}
        }
    });
}

fn load_window_icon() -> Result<window::Icon> {
    let bytes = include_bytes!("./assets/icons/icon-512.oxi.png");
    let img = image::load_from_memory_with_format(bytes, image::ImageFormat::PNG)?;
    let rgba = img.as_rgba8().unwrap();
    let dimensions = img.dimensions();

    window::Icon::from_rgba(rgba.clone().into_vec(), dimensions.0, dimensions.1)
        .context("Failed to create window icon!")
}
