extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use vulkano::instance::{RawInstanceExtensions, Instance};
use std::ffi::CString;
use vulkano::VulkanObject;
use vulkano::swapchain::Surface;

fn main() -> Result<(), String> {
    println!("Hello, world!");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("snow-64 alpha", 512, 512).position_centered().vulkan().build().map_err(|e| e.to_string())?;

    let instance_exts = window.vulkan_instance_extensions()?;
    let raw_instance_exts = RawInstanceExtensions::new(instance_exts.iter().map(|&v| CString::new(v).unwrap()));
    let instance = Instance::new(None, raw_instance_exts, None).unwrap();
    let surface_handle = window.vulkan_create_surface(instance.internal_object())?;
    let _surface = unsafe { Surface::from_raw_surface(instance, surface_handle, window.context()) };

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, 256, 256).map_err(|e| e.to_string())?;
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

    canvas.clear();
    canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;
    canvas.copy_ex(&texture, None, Some(Rect::new(450, 100, 256, 256)), 30.0, None, false, false)?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }

        println!("LULW");
    }

    Ok(())
}
