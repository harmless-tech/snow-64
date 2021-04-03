use lazy_static::lazy_static;
use sdl2::{
    pixels::PixelFormatEnum,
    rect::Rect,
    render::{BlendMode, Texture, TextureCreator, WindowCanvas},
    video::WindowContext,
};
use std::{
    borrow::{BorrowMut},
    sync::{Mutex},
};

lazy_static! {
    static ref LAYERS: Mutex<Layers> = Mutex::new(Layers {
        layers: vec![
            Layer::Layer0(vec![0_u8; LAYERS_SIZE]),
            Layer::Layer1(vec![0_u8; LAYERS_SIZE]),
            Layer::Layer2(vec![0_u8; LAYERS_SIZE]),
            Layer::Layer3(vec![0_u8; LAYERS_SIZE]),
            Layer::Layer4(vec![0_u8; LAYERS_SIZE]),
            Layer::Layer5(vec![0_u8; LAYERS_SIZE]),
        ],
        current_layer: 0,
        allow_pixel_layer: false,
    });
    static ref TILE_MAPS: Mutex<TileMaps> = Mutex::new(TileMaps {
        tile_maps: vec![
            TileMap::Map0(vec![0_u8; TILE_MAP_SIZE]),
            TileMap::Map1(vec![0_u8; TILE_MAP_SIZE]),
            TileMap::Map2(vec![0_u8; TILE_MAP_SIZE]),
            TileMap::Map3(vec![0_u8; TILE_MAP_SIZE]),
        ],
        current_map: 0,
        current_tile: 0,
    });
}

const LAYERS_WIDTH: u32 = 512;
const LAYERS_HEIGHT: u32 = 512;
const LAYERS_PITCH: usize = LAYERS_WIDTH as usize * 4;
const LAYERS_SIZE: usize = LAYERS_PITCH * LAYERS_HEIGHT as usize;
const LAYERS_RECT: (i32, i32, u32, u32) = (0, 0, LAYERS_WIDTH, LAYERS_HEIGHT);
const AMOUNT_LAYERS: usize = 6;

enum Layer {
    Layer0(Vec<u8>), // Tile.
    Layer1(Vec<u8>), // Entity.
    Layer2(Vec<u8>), // Tile.
    Layer3(Vec<u8>), // Entity.
    Layer4(Vec<u8>), // Text.
    Layer5(Vec<u8>), // Pixel.
}

struct Layers {
    layers: Vec<Layer>,
    current_layer: u32,
    allow_pixel_layer: bool,
}

const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;
const TILE_PITCH: usize = TILE_WIDTH as usize * 4;
const TILE_SIZE: usize = TILE_PITCH * TILE_HEIGHT as usize;
const AMOUNT_TILES: usize = 32;

const TILE_MAP_SIZE: usize = TILE_SIZE * 32;
const AMOUNT_TILE_MAPS: usize = 4;

enum TileMap {
    Map0(Vec<u8>),
    Map1(Vec<u8>),
    Map2(Vec<u8>),
    Map3(Vec<u8>),
}

struct TileMaps {
    tile_maps: Vec<TileMap>,
    current_map: u32,
    current_tile: u32,
}

pub fn init_textures(tex_creator: &TextureCreator<WindowContext>) -> Result<Vec<Texture>, String> {
    let mut textures = Vec::<Texture>::new();
    for _i in 0..AMOUNT_LAYERS {
        let mut tex = tex_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, LAYERS_WIDTH, LAYERS_HEIGHT)
            .map_err(|e| e.to_string())?;
        tex.set_blend_mode(BlendMode::Blend);
        textures.push(tex);
    }

    Ok(textures)
}

//TODO Allow tiles to be mapped to the buffer.
pub fn draw(canvas: &mut WindowCanvas, textures: &mut Vec<Texture>) -> Result<(), String> {
    canvas.clear();

    build_textures(textures.borrow_mut())?;
    let (width, height) = canvas.window().size();
    for tex in textures.iter() {
        canvas.copy(&tex, None, Some(Rect::new(0, 0, width, height)))?;
    }

    canvas.present();

    Ok(())
}

// All tile maps are loaded before program start.
pub fn load_tile_maps(maps: &Vec<TileMap>) {

}

fn build_textures(textures: &mut Vec<Texture>) -> Result<(), String> {
    if textures.len() == AMOUNT_LAYERS {
        let layers = LAYERS.lock().unwrap();

        for layer in layers.layers.iter() {
            match layer {
                Layer::Layer0(buffer) => proc_texture(0, textures.borrow_mut(), buffer)?,
                Layer::Layer1(buffer) => proc_texture(1, textures.borrow_mut(), buffer)?,
                Layer::Layer2(buffer) => proc_texture(2, textures.borrow_mut(), buffer)?,
                Layer::Layer3(buffer) => proc_texture(3, textures.borrow_mut(), buffer)?,
                Layer::Layer4(buffer) => proc_texture(4, textures.borrow_mut(), buffer)?,
                Layer::Layer5(buffer) => {
                    if layers.allow_pixel_layer {
                        proc_texture(5, textures.borrow_mut(), buffer)?;
                    }
                }
            }
        }
    }
    else {
        return Err(
            "Wrong number of texture layers were passed to the render function!".to_string(),
        );
    }

    Ok(())
}

fn proc_texture(index: usize, textures: &mut Vec<Texture>, buffer: &[u8]) -> Result<(), String> {
    match textures.get_mut(index) {
        None => return Err("Textures vector is missing a layer!".to_string()),
        Some(tex) => {
            tex.update(Rect::from(LAYERS_RECT), buffer, LAYERS_PITCH as usize)
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

pub mod commands {
}
