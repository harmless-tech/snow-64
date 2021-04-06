mod native;
mod scripting;

pub enum Program {
    Native(),
    NativeAddon(), //TODO This should probably just fall under native programs. Only the shell will have the ability to run other programs.
    Rhai(),
    Wren(),
    Typescript(),
    RuntimeAssembly(),
}
