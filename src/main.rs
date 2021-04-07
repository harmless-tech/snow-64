mod logging;
mod programs;
mod render;
mod update;

use game_loop;
use log::{debug, error, info, trace, warn};
use macroquad::prelude::*;
use std::{io::Cursor, time::Instant};

const WIDTH: f32 = 256.0;
const HEIGHT: f32 = 256.0;
const RES_SCALE: i32 = WIDTH as i32 * 4;
const FIXED_UPDATE_TIME: u128 = 16666667; //TODO This is not stable at all.

#[cfg(debug_assertions)]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

struct Snow64();

//TODO Allow for a config file to change settings.
// No window icon currently. :(
fn window_cfg() -> Conf {
    Conf {
        window_title: "Snow 64 - alpha build".to_string(),
        window_width: RES_SCALE,
        window_height: RES_SCALE,
        high_dpi: true,
        fullscreen: false,
        sample_count: 0,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_cfg)]
async fn main() {
    let mut args = std::env::args();
    let debug = DEBUG || args.any(|s| s.eq("--debug"));

    let _logging = logging::setup_log(debug);
    info!("Logging Check!");
    info!("Logging Level Info: TRUE");
    warn!("Logging Level Warn: TRUE");
    error!("Logging Level Error: TRUE");
    debug!("Logging Level Debug: TRUE");
    trace!("Logging Level Trace: TRUE");

    // Load
    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 256.0, 256.0));

    let img = image::load(
        Cursor::new(&include_bytes!("./assets/icon-256.png")[..]),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let img = Image {
        bytes: img.to_vec(),
        width: img.width() as u16,
        height: img.height() as u16,
    };
    let texture = load_texture_from_image(&img);
    set_texture_filter(texture, FilterMode::Nearest);
    let mut params = DrawTextureParams::default();
    params.dest_size = Some(Vec2::new(256.0, 256.0));
    params.source = Some(Rect::new(0.0, 0.0, 256.0, 256.0));
    let params = params;

    // Loop
    let mut run = true;
    let mut prev_fixed_update = Instant::now();

    let mut fixed_count_time = Instant::now();
    let mut fixed_count: u64 = 0;

    while run {
        // Fixed Update (Not really)
        if Instant::now().duration_since(fixed_count_time).as_nanos() >= 1_000_000_000 {
            fixed_count_time = Instant::now();
            debug!("\"Fixed\" Update: {}", fixed_count);
            fixed_count = 0;
        }

        if Instant::now().duration_since(prev_fixed_update).as_nanos() >= FIXED_UPDATE_TIME {
            prev_fixed_update = Instant::now();
            fixed_count += 1;
        }

        // Draw
        clear_background(BLACK);
        set_camera(camera);

        draw_texture_ex(texture, 0.0, 0.0, WHITE, params.clone());

        next_frame().await;
    }
}
