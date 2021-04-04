use rhai::{plugin::*, Module};

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
