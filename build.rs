use anyhow::*;
use glob::glob;
use rayon::prelude::*;
use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

struct ShaderData {
    src: String,
    src_path: PathBuf,
    spv_path: PathBuf,
    kind: shaderc::ShaderKind,
}
impl ShaderData {
    pub fn load(src_path: PathBuf) -> Result<Self> {
        let ext = src_path
            .extension()
            .context("File has no extension.")?
            .to_str()
            .context("Cannot convert path to &str.")?;
        let kind = match ext {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            "comp" => shaderc::ShaderKind::Compute,
            _ => bail!("Unsupported shader: {}", src_path.display()),
        };

        let src = read_to_string(src_path.clone())?;
        let spv_path = src_path.with_extension(format!("{}.spv", ext));

        Ok(Self {
            src,
            src_path,
            spv_path,
            kind,
        })
    }
}

struct ImageData {
    src_path: PathBuf,
    oxi_path: PathBuf,
}
impl ImageData {
    pub fn load(src_path: PathBuf) -> Result<Self> {
        let ext = src_path
            .extension()
            .context("File has no extension.")?
            .to_str()
            .context("Cannot convert path to &str.")?;

        let oxi_path = src_path.with_extension(format!("{}.oxi", ext));

        Ok(Self {
            src_path,
            oxi_path,
        })
    }
}

fn main() -> Result<()> {
    let mut shader_paths = Vec::new();
    shader_paths.extend(glob("./src/**/*.vert")?);
    shader_paths.extend(glob("./src/**/*.frag")?);
    shader_paths.extend(glob("./src/**/*.comp")?);

    let shaders = shader_paths
        .into_par_iter()
        .map(|g| ShaderData::load(g?))
        .collect::<Vec<Result<_>>>()
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    let mut compiler = shaderc::Compiler::new().context("Unable to create shader compiler.")?;

    for shader in shaders {
        println!(
            "cargo:rerun-if-changed={}",
            shader.src_path.as_os_str().to_str().unwrap()
        );

        let compiled = compiler.compile_into_spirv(
            &shader.src,
            shader.kind,
            &shader.src_path.to_str().unwrap(),
            "main",
            None,
        )?;
        write(shader.spv_path, compiled.as_binary_u8())?;
    }

    let mut image_paths = Vec::new();
    image_paths.extend(glob("./src/**/*.png")?);

    let images = image_paths
        .into_par_iter()
        .map(|g| ImageData::load(g?))
        .collect::<Vec<Result<_>>>()
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    let image_options = oxipng::Options::max_compression();

    for image in images {
        println!(
            "cargo:rerun-if-changed={}",
            image.src_path.as_os_str().to_str().unwrap()
        );

        oxipng::optimize(&oxipng::InFile::Path(image.src_path), &oxipng::OutFile::Path(Some(image.oxi_path)), &image_options)?;
    }

    // println!("cargo:rerun-if-changed=res/*");
    //
    // let out_dir = env::var("OUT_DIR")?;
    // let mut copy_options = CopyOptions::new();
    // copy_options.overwrite = true;
    // let mut paths_to_copy = Vec::new();
    // paths_to_copy.push("res/");
    // copy_items(&paths_to_copy, out_dir, &copy_options)?;

    Ok(())
}
