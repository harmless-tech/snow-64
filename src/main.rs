mod render;
mod scripting;
mod update;
mod programs;
mod program;
mod logging;

extern crate sdl2;

use log::{debug, error, info, trace, warn};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    rect::Rect,
    surface::Surface,
    video::{Window, WindowContext},
};
use std::{ffi::CString, io::Cursor, rc::Rc};
use rhai::{Engine, Scope, OptimizationLevel};

//
fn display(s: &str) {
    info!("[RHAI]: {}", s);
}
//

fn main() -> Result<(), String> {
    let _logging = logging::setup_log();
    info!("Logging Check!");

    info!("SDL2 Init.");
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    info!("Window Init.");
    let mut window = video_subsystem
        .window("snow-64 alpha", 512, 512)
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

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    info!("Using SDL_Renderer \"{}\"", canvas.info().name);
    info!("Start!");

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
        .map_err(|e| e.to_string())?;
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..256 {
            for x in 0..256 {
                let offset = y * pitch + x * 3;
                buffer[offset] = x as u8;
                buffer[offset + 1] = y as u8;
                buffer[offset + 2] = 0;
            }
        }
    })?;

    //
    let mut engine = Engine::new();
    let mut scope = Scope::new();

    engine.register_fn("display", display);

    let ast = engine.compile_file_with_scope(&scope, "./test/tes-game/entry.rhai".into()).unwrap();
    let new_ast = engine.optimize_ast(&scope, ast.clone(), OptimizationLevel::Full);

    engine.eval_ast::<()>(&new_ast);
    //

    canvas.present();

    /*let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }*/

    Ok(())
}

fn setup_window_icon(window: &mut Window) -> Result<(), String> {
    let mut img = image::load(
        Cursor::new(&include_bytes!("./assets/icon.png")[..]),
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
