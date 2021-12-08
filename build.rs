use anyhow::*;
use glob::glob;
use rayon::prelude::*;
use std::path::PathBuf;

struct ImageData {
    src_path: PathBuf,
    oxi_path: PathBuf,
}
impl ImageData {
    pub fn load(src_path: PathBuf) -> Result<Self> {
        let _ext = src_path
            .extension()
            .context("File has no extension.")?
            .to_str()
            .context("Cannot convert path to &str.")?;

        let s = src_path.as_os_str().to_str().unwrap();
        let path = &s[..(s.len() - 4)];

        let oxi_path = PathBuf::from(format!("{}.oxi.png", path));

        Ok(Self { src_path, oxi_path })
    }
}

fn main() -> Result<()> {
    let mut image_paths = Vec::new();
    image_paths.extend(glob("./src/**/*.png")?);

    let images = image_paths
        .into_par_iter()
        .map(|g| ImageData::load(g?))
        .collect::<Vec<Result<_>>>()
        .into_iter()
        .collect::<Result<Vec<_>>>()?;

    let mut image_options = oxipng::Options::max_compression();
    image_options.color_type_reduction = false;

    for image in images {
        if !image
            .src_path
            .as_os_str()
            .to_str()
            .unwrap()
            .ends_with(".oxi.png")
        {
            println!(
                "cargo:rerun-if-changed={}",
                image.src_path.as_os_str().to_str().unwrap()
            );

            //TODO Should the image be replaced in the future?
            oxipng::optimize(
                &oxipng::InFile::Path(image.src_path),
                &oxipng::OutFile::Path(Some(image.oxi_path)),
                &image_options,
            )?;
        }
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
