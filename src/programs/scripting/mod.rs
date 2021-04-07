pub mod rhai_script;

use crate::programs::Program;
use log::error;

// Hmmmmmm
pub fn run_program(program: Program) {
    match program {
        Program::Native() => {}
        Program::NativeAddon() => {} // Nah
        Program::Rhai() => {}
        Program::Wren() => {
            error!("Not yet!");
        }
        Program::Typescript() => {
            error!("Not yet!");
        }
        Program::RuntimeAssembly() => {
            error!("Not yet!");
        }
    }
}
