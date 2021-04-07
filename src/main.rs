mod logging;
mod programs;
mod render;
mod update;

use log::{debug, error, info, trace, warn};
use macroquad::prelude::*;
use std::{io::Cursor, time::Instant};
use configparser::ini::Ini;
use macroquad::telemetry::{enable, scene_allocated_memory};

const WIDTH: f32 = 256.0;
const HEIGHT: f32 = 256.0;
const RES_SCALE: i32 = WIDTH as i32 * 4;
const FIXED_UPDATE_TIME: u128 = 16666667; //TODO This is not a stable 60 fps at all.

#[cfg(debug_assertions)]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

struct Snow64();

struct SnowConfig {
    display_wh: u32,
    display_msaa: u32,
    dev_debug: bool,
}
impl Default for SnowConfig {
    fn default() -> Self {
        SnowConfig {
            display_wh: RES_SCALE as u32,
            display_msaa: 0,
            dev_debug: DEBUG,
        }
    }
}

#[macroquad::main(window_cfg)]
async fn main() {
    let config = load_config();
    let mut args = std::env::args();
    let debug = DEBUG || config.dev_debug || args.any(|s| s.eq("--debug"));

    let _logging = logging::setup_log(debug);
    info!("Logging Check!");
    info!("Logging Level Info: TRUE");
    warn!("Logging Level Warn: TRUE");
    error!("Logging Level Error: TRUE");
    debug!("Logging Level Debug: TRUE");
    trace!("Logging Level Trace: TRUE");

    // Load
    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 256.0, 256.0));

    // let img = image::load(
    //     Cursor::new(&include_bytes!("./assets/icon-256.png")[..]),
    //     image::ImageFormat::Png,
    // )
    // .unwrap()
    // .to_rgba8();
    // let img = Image {
    //     bytes: img.to_vec(),
    //     width: img.width() as u16,
    //     height: img.height() as u16,
    // };
    // let texture = load_texture_from_image(&img);
    // set_texture_filter(texture, FilterMode::Nearest);

    let mut textures = render::init_textures();

    let mut params = DrawTextureParams::default();
    params.dest_size = Some(Vec2::new(256.0, 256.0));
    params.source = Some(Rect::new(0.0, 0.0, 256.0, 256.0));
    let params = params;

    // Loop Logic
    let mut run = true;
    let mut prev_fixed_update = Instant::now();

    let mut fixed_count_time = Instant::now();
    let mut fixed_count: u64 = 0;

    // Main Loop
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

        render::draw(&mut textures, &params);

        // draw_texture_ex(texture, 0.0, 0.0, WHITE, params.clone());
        // render::draw(&params);

        enable();
        debug!("Mem: {}", scene_allocated_memory());

        next_frame().await;
    }
}

//TODO Allow for a config file to change settings.
// No window icon currently. :(
fn window_cfg() -> Conf {
    let config = load_config();

    Conf {
        window_title: "Snow 64 - alpha build".to_string(),
        window_width: config.display_wh as i32,
        window_height: config.display_wh as i32,
        high_dpi: true,
        fullscreen: false,
        sample_count: config.display_msaa as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn load_config() -> SnowConfig {
    let mut config = SnowConfig::default();

    let file = std::fs::read_to_string("./snow-64-data/config.ini").unwrap_or("".to_string());
    if !file.eq("") {
        let mut parser = Ini::new();
        parser.read(file.clone()).expect("Could not parse config.ini!");

        config.display_wh = parser.getuint("display", "wh").unwrap_or(Some(RES_SCALE as u64)).unwrap_or(RES_SCALE as u64) as u32;
        config.display_msaa = parser.getuint("display", "msaa").unwrap_or(Some(0)).unwrap_or(0) as u32;
        config.dev_debug = parser.getbool("dev", "debug").unwrap_or(Some(DEBUG)).unwrap_or(DEBUG);
    }

    config
}
