mod native;
mod scripting;

pub enum Program {
    Native(),
    NativeAddon(),
    Rhai(),
    Wren(),
    Typescript(),
    RuntimeAssembly(),
}
