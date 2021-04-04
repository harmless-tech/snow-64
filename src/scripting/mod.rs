mod rhai_script;

use log::{debug, info};
use std::rc::Rc;

pub fn run_rhai_program(path: &str) -> Result<(), String> {
    use crate::scripting::rhai_script::generate_snow_module;
    use rhai::{Engine, Module, OptimizationLevel, Scope};

    let mut engine = Engine::new(); // _raw
    engine.set_optimization_level(OptimizationLevel::Full);
    engine.on_print(|s| info!("[RHAI]: {}", s));

    let module = generate_snow_module()?;
    engine.register_global_module(Rc::from(module));

    // let mod_scope = Scope::new();
    // let mod_ast = engine.compile_file_with_scope(&mod_scope, "./test/tes-game/entity.rhai".into()).unwrap();
    // let entity = Module::eval_ast_as_new(mod_scope, &mod_ast, &engine).map_err(|e| e.to_string())?;
    // engine.register_static_module("entity", Rc::from(entity));

    engine.set_module_resolver(rhai::module_resolvers::FileModuleResolver::new_with_path(
        "./test/tes-game",
    ));

    debug!("Wow!");

    let scope = Scope::new();
    let ast = engine.compile_file_with_scope(&scope, path.into()).unwrap();
    engine.eval_ast::<()>(&ast).map_err(|e| e.to_string())?; //TODO

    Ok(())
}
