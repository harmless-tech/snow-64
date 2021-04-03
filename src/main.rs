mod logging;
mod program;
mod programs;
mod render;
mod scripting;
mod update;

extern crate sdl2;

use log::{debug, error, info, trace, warn};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::PixelFormatEnum,
    surface::Surface,
    video::{Window, WindowContext, WindowPos},
};
use std::{
    borrow::{Borrow, BorrowMut},
    ffi::CString,
    io::Cursor,
    rc::Rc,
};

fn init() -> Result<(), String> {
    Ok(())
}

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

    info!("Renderer Init.");
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let mut textures = render::init_textures(&texture_creator)?;

    info!("Using SDL_Renderer \"{}.\"", canvas.info().name);
    info!("Start!");

    //TODO Remove!
    let mut v = vec![0_u8; 128];
    debug!("{:?}", v);
    v.splice(0..10, [1_u8; 10].iter().cloned());
    debug!("{:?}", v);
    // /\

    render::draw(&mut canvas, textures.borrow_mut())?; //TODO Remove! (First and only draw)

    let mut event_pump = sdl_context.event_pump()?;

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
                        .set_size(width + 8, height + 8)
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
                        .set_size(width - 8, height - 8)
                        .map_err(|e| e.to_string())?;
                    canvas
                        .window_mut()
                        .set_position(WindowPos::Centered, WindowPos::Centered);

                    render::draw(&mut canvas, textures.borrow_mut())?;
                }
                _ => {}
            }
        }
    }

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
