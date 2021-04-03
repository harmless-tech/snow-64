use lazy_static::lazy_static;
use sdl2::{
    pixels::PixelFormatEnum,
    rect::Rect,
    render::{Texture, TextureCreator, WindowCanvas},
    surface::SurfaceContext,
    video::WindowContext,
};
use std::{
    borrow::{Borrow, BorrowMut},
    io::Cursor,
    sync::Mutex,
};
use sdl2::render::BlendMode;

lazy_static! {
    static ref SETTINGS: Mutex<RenderSettings> = Mutex::new(RenderSettings {
        curr_tile_map: TileMap::Map0,
        curr_tile: 0,
        allow_pixel_editing: false,
    });
    static ref BUFFER: Mutex<Buffers> = Mutex::new(Buffers {
        layer_0: vec![0_u8; BUFFER_SIZE],
        layer_1: vec![0_u8; BUFFER_SIZE],
        layer_2: vec![0_u8; BUFFER_SIZE],
        layer_3: vec![0_u8; BUFFER_SIZE],
        layer_4: vec![0_u8; BUFFER_SIZE],
    });
    static ref TILE_MAPS: Mutex<TileMaps> = Mutex::new(TileMaps {
        map_0: vec![0_u8; TILE_MAP_SIZE],
        map_1: vec![0_u8; TILE_MAP_SIZE],
        map_2: vec![0_u8; TILE_MAP_SIZE],
        map_3: vec![0_u8; TILE_MAP_SIZE],
    });
}

struct RenderSettings {
    curr_layer_1_mode: LayerMode,
    curr_tile_map: TileMap,
    curr_tile: usize,
    allow_pixel_editing: bool,
}

const BUFFER_WIDTH: u32 = 512;
const BUFFER_HEIGHT: u32 = 512;
const BUFFER_PITCH: usize = BUFFER_WIDTH as usize * 4;
const BUFFER_SIZE: usize = BUFFER_PITCH * BUFFER_HEIGHT as usize;
const BUFFER_RECT: (i32, i32, u32, u32) = (0, 0, BUFFER_WIDTH, BUFFER_HEIGHT);

struct Buffers/*(Vec<Layers>);*/
{
    layer_0: Vec<u8>, // Tile Layer.
    layer_1: Vec<u8>, // Tile/Entity Layer.
    layer_2: Vec<u8>, // Tile/Entity Layer.
    layer_3: Vec<u8>, // Entity Layer.
    layer_4: Vec<u8>, // Text Layer.
    layer_5: Vec<u8>, // Pixel Layer.
}

const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;
const TILE_PITCH: usize = TILE_WIDTH as usize * 4;
const TILE_SIZE: usize = TILE_PITCH * TILE_HEIGHT as usize;

const TILE_MAP_SIZE: usize = TILE_SIZE * 32;

struct TileMaps {
    map_0: Vec<u8>,
    map_1: Vec<u8>,
    map_2: Vec<u8>,
    map_3: Vec<u8>,
}

enum Layers {
    Layer0(Vec<u8>),
    Layer1(Vec<u8>),
    Layer2(Vec<u8>),
    Layer3(Vec<u8>),
    Layer4(Vec<u8>),
}

enum TileMap {
    Map0,
    Map1,
    Map2,
    Map3,
}

const AMOUNT_TEXTURES: usize = 5;

pub fn init_textures(tex_creator: &TextureCreator<WindowContext>) -> Result<Vec<Texture>, String> {
    let mut textures = Vec::<Texture>::new();
    for _i in 0..AMOUNT_TEXTURES {
        let mut tex = tex_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, BUFFER_WIDTH, BUFFER_HEIGHT)
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

fn build_textures(textures: &mut Vec<Texture>) -> Result<(), String> {
    if textures.len() == AMOUNT_TEXTURES {
        textures
            .get_mut(0)
            .unwrap()
            .update(
                Rect::from(BUFFER_RECT),
                BUFFER.lock().unwrap().layer_0.borrow(),
                BUFFER_PITCH as usize,
            )
            .map_err(|e| e.to_string())?;
        textures
            .get_mut(1)
            .unwrap()
            .update(
                Rect::from(BUFFER_RECT),
                BUFFER.lock().unwrap().layer_1.borrow(),
                BUFFER_PITCH as usize,
            )
            .map_err(|e| e.to_string())?;
        textures
            .get_mut(2)
            .unwrap()
            .update(
                Rect::from(BUFFER_RECT),
                BUFFER.lock().unwrap().layer_2.borrow(),
                BUFFER_PITCH as usize,
            )
            .map_err(|e| e.to_string())?;
        textures
            .get_mut(3)
            .unwrap()
            .update(
                Rect::from(BUFFER_RECT),
                BUFFER.lock().unwrap().layer_3.borrow(),
                BUFFER_PITCH as usize,
            )
            .map_err(|e| e.to_string())?;
        textures
            .get_mut(4)
            .unwrap()
            .update(
                Rect::from(BUFFER_RECT),
                BUFFER.lock().unwrap().layer_4.borrow(),
                BUFFER_PITCH as usize,
            )
            .map_err(|e| e.to_string())?;
    }
    else {
        return Err(
            "Wrong number of texture layers were passed to the render function!".to_string(),
        );
    }

    Ok(())
}

pub mod commands {}
