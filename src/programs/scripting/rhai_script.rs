use log::{debug, info};
use rhai::{plugin::*, Engine, Module, OptimizationLevel, Scope};
use std::rc::Rc;

//TODO Clean!
pub fn create_rhai_program(path: &str) -> Result<rhai::AST, String> {
    let mut engine = Engine::new_raw();
    engine.set_optimization_level(OptimizationLevel::Full);

    let module = generate_snow_module()?;
    engine.register_global_module(Rc::from(module));

    engine.set_module_resolver(rhai::module_resolvers::FileModuleResolver::new_with_path(
        "./test/tes-game",
    ));

    debug!("Wow!");

    let scope = Scope::new();
    let ast = engine.compile_file_with_scope(&scope, path.into()).unwrap();
    // engine.eval_ast::<()>(&ast).map_err(|e| e.to_string())?;

    Ok(ast)
}

pub fn generate_snow_module() -> Result<Module, String> {
    let mut module = Module::new();

    {
        use crate::render::commands::*;

        set_exported_global_fn!(module, "create_color", create_color);

        set_exported_global_fn!(module, "enable_pixel_layer", enable_pixel_layer);
        set_exported_global_fn!(module, "disable_pixel_layer", disable_pixel_layer);
    }

    Ok(module)
}
