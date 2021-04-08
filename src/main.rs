mod logging;
mod programs;
mod render;
mod update;

use bevy::{
    core::FixedTimestep,
    prelude::*,
    window::{WindowMode, WindowPlugin, WindowResized},
};
use log::{debug, error, info, trace, warn};
use crate::programs::scripting::rhai_script;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

const WIDTH: f32 = 256.0;
const HEIGHT: f32 = 256.0;

#[cfg(debug_assertions)]
const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG: bool = false;

const FIXED_UPDATE_LABEL: &str = "FIXED_UPDATE_STAGE";

#[derive(Debug, Hash, Eq, PartialEq, Clone, StageLabel)]
struct FixedUpdateStage;

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

    App::build()
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            resize_constraints: Default::default(),
            scale_factor_override: None,
            title: "Snow64 - alpha build".to_string(),
            vsync: false,
            resizable: false,
            decorations: true,
            cursor_visible: true,
            cursor_locked: false,
            mode: WindowMode::Windowed,
        })
        .insert_resource(ClearColor(Color::rgba(0.0, 0.0, 0.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_stage_after(
            CoreStage::Update,
            FixedUpdateStage,
            SystemStage::parallel().with_run_criteria(
                FixedTimestep::steps_per_second(60.0).with_label(FIXED_UPDATE_LABEL),
            ),
        )
        .add_plugin(rhai_script::RhaiProgramPlugin)
        .run();

    Ok(())
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn().insert_bundle(OrthographicCameraBundle::new_2d());

    SpriteBundle {

    }
}
