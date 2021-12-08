mod logging;
mod config;

use anyhow::*;
use image::GenericImageView;
use log::{debug, error, info, trace, warn};
use std::time::Instant;
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
 *
 *
 * TODO Maybe?
 * Have a big buffer in the program.
 * Upload a small part of it to the gpu.
 */

struct Snow64; //TODO

fn main() -> Result<()> {
    let config = config::load_config()?;
    let _logging = logging::setup_log(config.dev_debug)?;

    let event_loop: EventLoop<FixedLoopEvent> = EventLoop::with_user_event();
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
        .with_resizable(false)
        .with_window_icon(Some(load_window_icon()?))
        .with_title("Snow64 - alpha build")
        .build(&event_loop)
        .unwrap();

    // Pixels setup

    // For DBG, remove later
    let mut fps_counter: u64 = 0;
    let mut last_time = Instant::now();
    //

    start_fixed_loop_thread(event_loop.create_proxy());

    //TODO: If statements might look nicer?
    event_loop.run(move |event, _, control_flow| match event {
        Event::UserEvent(FixedLoopEvent) => {
            //TODO: Update.
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
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
        Event::MainEventsCleared => window.request_redraw(),
        _ => {}
    });
}

fn load_window_icon() -> Result<window::Icon> {
    let bytes = include_bytes!("./assets/icons/icon-512.oxi.png");
    let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?;
    let rgba = img.as_rgba8().unwrap();
    let dimensions = img.dimensions();

    window::Icon::from_rgba(rgba.clone().into_vec(), dimensions.0, dimensions.1)
        .context("Failed to create window icon!")
}

const FIXED_LOOP_TIME: u64 = 16666667 - 2500000;
struct FixedLoopEvent;
fn start_fixed_loop_thread(event: winit::event_loop::EventLoopProxy<FixedLoopEvent>) {
    let time = std::time::Duration::from_nanos(FIXED_LOOP_TIME);

    std::thread::spawn(move || {
        // For DBG, remove later
        let mut fixed_counter: u64 = 0;
        let mut last_time = Instant::now();
        //

        'fixed: loop {
            std::thread::sleep(time);

            match event.send_event(FixedLoopEvent) {
                Ok(_) => {}
                Err(_) => {
                    break 'fixed;
                }
            }

            // For DBG, remove later
            fixed_counter += 1;
            if last_time.elapsed().as_secs() >= 1 {
                last_time = Instant::now();
                info!("Fixed (Internal): {}", fixed_counter);
                fixed_counter = 0;
            }
            //
        }
    });
}
