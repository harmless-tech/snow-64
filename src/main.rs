mod logging;
mod programs;
mod render;
mod update;

extern crate sdl2;

use game_loop::game_loop;
use image::{EncodableLayout, RgbaImage};
use log::{debug, error, info, trace, warn};
use sdl2::{event::Event, keyboard::{Keycode, Mod}, pixels::PixelFormatEnum, render::BlendMode, surface::Surface, video::{Window, WindowContext, WindowPos}, Sdl};
use std::{
    borrow::{Borrow, BorrowMut},
    ffi::CString,
    io::Cursor,
    rc::Rc,
};
use sdl2::render::{WindowCanvas, Texture};
use std::cmp::max;
use sdl2::rect::Rect;
use crate::render::colors::WHITE;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

#[cfg(debug_assertions)]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

struct Snow64(); // Should hold program data?
impl Snow64 {
    fn new() -> Self {
        Snow64()
    }

    fn fixed_update(&self, time_step: f64) {
    }

    fn update(&self, time_step: f64) {
    }

    fn draw(&self, time_step: f64) {
    }

    fn event(&self, event: Event) {
    }
}

fn init(sdl_context: &Sdl, canvas: &mut WindowCanvas, textures: &mut Vec<Texture>, debug: bool) -> Result<(), String> {
    let mut snow64 = Snow64::new();
    let mut event_pump = sdl_context.event_pump()?;

    render::commands::toggle_pixel_layer();
    // render::commands::draw_pixel(255, 255, WHITE);
    for x in 0..(256) {
        for y in 0..(256) {
            render::commands::draw_pixel(x, y, WHITE);
        }
    }
    // render::draw(canvas, textures);

    game_loop(snow64, 60, 0.1, |snow| {
        // Fixed Update
    }, |snow| {
        // Events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => snow.exit(),
                Event::KeyDown {
                    keycode: Some(key),
                    keymod: m,
                    ..
                } => {
                    debug!("mod: {}", m);
                    match (key, (m & Mod::LCTRLMOD) == Mod::LCTRLMOD) {
                        (Keycode::Q, true) => snow.exit(),
                        (_, _) => {}
                    }
                },
                Event::TextInput {
                    text: input,
                    ..
                } => {
                    debug!("text: {}", input);
                }
                _ => {}
            }
        }

        // Update
        // Draw
    });

    Ok(())
}

fn main() -> Result<(), String> {
    let mut args = std::env::args();
    let debug = DEBUG || args.any(|s| s.eq("--debug"));

    let _logging = logging::setup_log(debug);
    info!("Logging Check!");
    info!("Logging Level Info: TRUE");
    warn!("Logging Level Warn: TRUE");
    error!("Logging Level Error: TRUE");
    debug!("Logging Level Debug: TRUE");
    trace!("Logging Level Trace: TRUE");

    info!("SDL2 Init.");
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    info!("Window Init.");
    let mut window = video_subsystem
        .window("snow-64 alpha build", 1024, 1024)
        .position_centered()
        .vulkan()
        .allow_highdpi()
        .build()
        .map_err(|e| e.to_string())?;

    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "nearest");
    setup_window_icon(&mut window)?;

    let window = window;

    info!("Vulkan Init.");
    let _surface = setup_vulkan(&window)?;

    info!("Renderer Init.");
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_blend_mode(BlendMode::Blend);
    canvas.set_viewport(Some(Rect::new(0, 0, WIDTH, HEIGHT)));
    canvas.set_logical_size(WIDTH, HEIGHT).map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut textures = render::init_textures(&texture_creator)?;

    info!("Using SDL_Renderer \"{}.\"", canvas.info().name);
    info!("Boot!");
    render::load_image_into_layer(0, get_icon().as_bytes(), WIDTH as usize, HEIGHT as usize);
    render::draw(&mut canvas, textures.borrow_mut())?;

    info!("Start!");
    init(&sdl_context, &mut canvas, &mut textures, debug)?;

    /*let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let (width, height) = canvas.window().size();
                    canvas
                        .window_mut()
                        .set_size(width * 2, height * 2)
                        .map_err(|e| e.to_string())?;
                    canvas
                        .window_mut()
                        .set_position(WindowPos::Centered, WindowPos::Centered);

                    render::draw(&mut canvas, textures.borrow_mut())?;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let (width, height) = canvas.borrow().window().size();
                    canvas
                        .window_mut()
                        .set_size(width / 2, height / 2)
                        .map_err(|e| e.to_string())?;
                    canvas
                        .window_mut()
                        .set_position(WindowPos::Centered, WindowPos::Centered);

                    render::draw(&mut canvas, textures.borrow_mut())?;
                }
                _ => {}
            }
        }
    }*/

    Ok(())
}

fn get_icon() -> RgbaImage {
    image::load(
        Cursor::new(&include_bytes!("./assets/icon-256.png")[..]),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8()
}

fn setup_window_icon(window: &mut Window) -> Result<(), String> {
    let mut img = image::load(
        Cursor::new(&include_bytes!("./assets/icon-512.png")[..]),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();

    let width = img.width();
    let height = img.height();

    let window_icon = Surface::from_data(
        img.as_mut(),
        width,
        height,
        width * 4,
        PixelFormatEnum::RGBA32,
    )?;
    window.set_icon(window_icon);

    Ok(())
}

fn setup_vulkan(window: &Window) -> Result<vulkano::swapchain::Surface<Rc<WindowContext>>, String> {
    use vulkano::{
        instance::{Instance, RawInstanceExtensions},
        swapchain::Surface,
        VulkanObject,
    };

    let instance_exts = window.vulkan_instance_extensions()?;
    let raw_instance_exts =
        RawInstanceExtensions::new(instance_exts.iter().map(|&v| CString::new(v).unwrap()));
    let instance = Instance::new(None, raw_instance_exts, None).unwrap();
    let surface_handle = window.vulkan_create_surface(instance.internal_object())?;
    Ok(unsafe { Surface::from_raw_surface(instance, surface_handle, window.context()) })
}
