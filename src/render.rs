use lazy_static::lazy_static;
use sdl2::{rect::Rect, render::Texture};
use std::{borrow::Borrow, sync::Mutex};

lazy_static! {
    static ref SETTINGS: Mutex<RenderSettings> = Mutex::new(RenderSettings {
        curr_layer_1_mode: LayerMode::Tile,
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

//TODO Maybe just use a vector of vectors.
struct Buffers {
    layer_0: Vec<u8>, // Background Layer.
    layer_1: Vec<u8>, // Tile/Entity Layer.
    layer_2: Vec<u8>, // Entity Layer.
    layer_3: Vec<u8>, // Text Layer.
    layer_4: Vec<u8>, // Pixel Layer.
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

enum LayerMode {
    Tile,
    Entity,
}

enum TileMap {
    Map0,
    Map1,
    Map2,
    Map3,
}

pub fn render(textures: &mut Vec<Texture>) -> Result<(), String> {
    if textures.len() == 5 {
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
