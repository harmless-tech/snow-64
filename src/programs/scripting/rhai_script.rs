use crate::{FIXED_UPDATE_LABEL, FixedUpdateStage};
use bevy::prelude::*;
use lazy_static::lazy_static;
use log::{debug, info};
use rhai::{
    module_resolvers::FileModuleResolver, plugin::*, Engine, Module, OptimizationLevel, Scope,
    Shared, AST,
};
use std::{
    path::{Path, PathBuf},
    process::exit,
    sync::{Mutex, MutexGuard},
};

lazy_static! {
    static ref ENGINE: Mutex<Option<Engine>> = Mutex::new(None);
}

struct RhaiProgram {
    ast: Option<AST>,
    fixed_update: bool,
    update: bool,
}

pub struct RhaiProgramPlugin;
impl Plugin for RhaiProgramPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(RhaiProgram {
            //engine,
            ast: None,
            fixed_update: false,
            update: false,
        })
        .add_event::<AddRhaiProgram>()
        .add_system_to_stage(FixedUpdateStage, call_fixed_updates.system())
        .add_system(call_updates.system());
    }
}

pub struct AddRhaiProgram(String); // Path to entry.
pub struct RemoveRhaiProgram; // Path to entry.

fn add_rhai_program(mut reader: EventReader<AddRhaiProgram>, mut program: ResMut<RhaiProgram>) {
    debug!("add!");

    for r in reader.iter() {
        let path = Path::new(&r.0);
        if path.is_file() {
            let dir = path.parent().unwrap();
            let engine = create_engine(dir.to_str().unwrap());

            let scope = Scope::new();
            let ast = match engine.compile_file_with_scope(&scope, path.to_path_buf()) {
                Ok(ast) => ast,
                Err(err) => {
                    error!("Could not compile program! (0) {}", err.to_string());
                    break;
                }
            };

            match engine.eval_ast::<()>(&ast) {
                Ok(_) => {}
                Err(err) => {
                    error!("Could not compile program! (1) {}", err.to_string());
                    break;
                }
            }

            //TODO Check for update and fixed_update! Then update struct.

            let mut e = ENGINE.lock().unwrap();
            let dest = mem::replace(&mut *e, Some(engine)); //TODO This work?
            mem::drop(dest);

            program.ast = Some(ast);
            program.fixed_update = false;
            program.update = false;
        }
        else {
            error!("Path was not a file! {:?}", path);
        }
    }
}

fn remove_rhai_program(
    mut reader: EventReader<RemoveRhaiProgram>,
    mut program: ResMut<RhaiProgram>,
) {
    debug!("remove!");

    for r in reader.iter() {
        let mut e = ENGINE.lock().unwrap();
        let dest = mem::replace(&mut *e, None); //TODO This work?
        mem::drop(dest);

        program.ast = None;
        program.fixed_update = false;
        program.update = false;
    }
}

fn call_fixed_updates(mut program: ResMut<RhaiProgram>) {
    if program.fixed_update {}
}

fn call_updates(mut program: ResMut<RhaiProgram>) {
    if program.update {}
}

fn create_engine(path: &str) -> Engine {
    let mut engine = Engine::new_raw();
    engine.set_optimization_level(OptimizationLevel::Full);

    let module = match generate_snow_module() {
        Ok(module) => module,
        Err(err) => {
            error!("Could not create snow module for rhai. {}", err);
            panic!();
        }
    };
    engine.register_global_module(Shared::new(module));

    engine.set_module_resolver(FileModuleResolver::new_with_path(path));

    engine
}

fn generate_snow_module() -> Result<Module, String> {
    let mut module = Module::new();

    {
        use crate::render::commands::*;

        set_exported_global_fn!(module, "create_color", create_color);

        /*set_exported_global_fn!(module, "enable_pixel_layer", enable_pixel_layer);
        set_exported_global_fn!(module, "disable_pixel_layer", disable_pixel_layer);*/
    }

    Ok(module)
}

// OLD
//TODO Clean!
//
// pub fn create_rhai_program(path: &str) -> Result<rhai::AST, String> {
//     let mut engine = Engine::new_raw();
//     engine.set_optimization_level(OptimizationLevel::Full);
//
//     let module = generate_snow_module()?;
//     engine.register_global_module(Rc::from(module));
//
//     engine.set_module_resolver(rhai::module_resolvers::FileModuleResolver::new_with_path(
//         "./test/tes-game",
//     ));
//
//     debug!("Wow!");
//
//     let scope = Scope::new();
//     let ast = engine.compile_file_with_scope(&scope, path.into()).unwrap();
//     // engine.eval_ast::<()>(&ast).map_err(|e| e.to_string())?;
//
//     Ok(ast)
// }
//

