mod config;
mod logging;
mod utils;

use crate::utils::blend;
use anyhow::*;
use image::{GenericImage, GenericImageView, Pixel};
use log::{debug, error, info, trace, warn};
use pixels::{Pixels, SurfaceTexture};
use rayon::prelude::*;
use std::{borrow::Borrow, thread, time::Instant};
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
    let _logging = logging::setup_log()?;

    let event_loop = EventLoop::new();
    //let mut input = WinitInputHelper::new(); TODO: Use this?
    let window = {
        WindowBuilder::new()
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
            .unwrap()
    };
    let main_window_id = window.id();

    // Pixels setup
    let layer1 = image::load_from_memory_with_format(
        &[0_u8; 256 /* Const for this. */ * 2 * 4],
        image::ImageFormat::Png,
    );
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(256, 256, surface_texture)?
    };
    let mut pixels_test = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(256, 256, surface_texture)?
    };

    let bytes = include_bytes!("./assets/icons/icon-256.oxi.png");
    let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png).unwrap();
    let img_data = img.to_rgba8();

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

                //TODO Update

                //TODO Draw
                test_draw(pixels.get_frame(), img_data.as_raw());

                for pix in pixels_test.get_frame().chunks_exact_mut(4) {
                    pix.copy_from_slice(&[0xff, 0x00, 0x00, 0xff]);
                }

                pixels.get_frame()
                    .par_chunks_exact_mut(4)
                    .zip(pixels_test.get_frame().par_chunks_exact(4))
                    .enumerate()
                    .for_each(|(i, (pixel, ipixel))| {
                        pixel[3] = 0x66;
                        pixel.copy_from_slice(&blend(ipixel, pixel));
                    });

                if pixels.render().is_err() {
                    error!("Pixels failed to render.");
                    *control_flow = ControlFlow::Exit;
                }
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
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::A),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    WindowEvent::Resized(size) => {
                        pixels.resize_surface(size.width, size.height);
                    }
                    _ => {}
                }
                //TODO: There should be a pass to other input handlers here.
            }
            _ => {}
        }
    });
}

//TODO Remove
fn test_draw(frame: &mut [u8], img_data: &[u8]) {
    let z_iter = frame
        .par_chunks_exact_mut(4)
        .zip(img_data.par_chunks_exact(4))
        .enumerate();
    z_iter.for_each(|(i, (pixel, ipixel))| {
        let mut p = ipixel.to_vec();
        p[3] = 0x00;

        // let inside = x % 4 == 0 && y % 4 == 0;
        // let rgba = if inside { [0x00, 0x00, 0x00, 0xff] } else { [0x48, 0x48, 0x48, 0x55] };
        pixel.copy_from_slice(ipixel);
    });

    // for (pixel, ipixel) in z_iter.par_iter_mut() {
    //     let mut p = ipixel.to_vec();
    //     p[3] = 0x66;
    //
    //     // let inside = x % 4 == 0 && y % 4 == 0;
    //     // let rgba = if inside { [0x00, 0x00, 0x00, 0xff] } else { [0x48, 0x48, 0x48, 0x55] };
    //     pixel.copy_from_slice(ipixel);
    // }
}
//

fn load_window_icon() -> Result<window::Icon> {
    let bytes = include_bytes!("./assets/icons/icon-512.oxi.png");
    let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?;
    let rgba = img.as_rgba8().unwrap();
    let dimensions = img.dimensions();

    window::Icon::from_rgba(rgba.clone().into_vec(), dimensions.0, dimensions.1)
        .context("Failed to create window icon!")
}
