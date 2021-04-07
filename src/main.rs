mod logging;
mod programs;
mod render;
mod update;

use crate::render::colors::WHITE;
use game_loop::game_loop;
use image::{EncodableLayout, RgbaImage};
use log::{debug, error, info, trace, warn};
use std::{
    borrow::{Borrow, BorrowMut},
    cmp::max,
    ffi::CString,
    io::Cursor,
    rc::Rc,
};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

#[cfg(debug_assertions)]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

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

    Ok(())
}
