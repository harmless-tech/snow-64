use lazy_static::lazy_static;
use log::{debug, error};
use sdl2::{
    gfx::primitives::DrawRenderer,
    pixels::{Color, PixelFormatEnum},
    rect::Rect,
    render::{BlendMode, Texture, TextureCreator, WindowCanvas},
    video::WindowContext,
};
use std::{borrow::BorrowMut, sync::Mutex};
use std::collections::HashMap;

lazy_static! {
    static ref LAYERS: Mutex<Layers> = Mutex::new(Layers {
        layers: [
            vec![0_u8; LAYER_SIZE],
            vec![0_u8; LAYER_SIZE],
            vec![0_u8; LAYER_SIZE],
            vec![0_u8; LAYER_SIZE],
            vec![0_u8; LAYER_SIZE],
            vec![0_u8; LAYER_SIZE]
        ],
        current_entity_layer: 0,
        current_tile_layer: 0,
        allow_pixel_layer: false,
    });
    static ref ENTITIES: Mutex<Entities> = Mutex::new(Entities {
        sprite_map: vec![0_u8; SPRITE_MAP_SIZE],
        e_map: HashMap::with_capacity(MAX_ENTITIES as usize),
    });
    static ref TILE_MAPS: Mutex<TileMaps> = Mutex::new(TileMaps {
        tile_maps: [
            vec![0_u8; TILE_MAP_SIZE],
            vec![0_u8; TILE_MAP_SIZE],
            vec![0_u8; TILE_MAP_SIZE],
            vec![0_u8; TILE_MAP_SIZE]
        ],
        current_map: 0,
        current_tile: 0,
    });
    static ref FONT_MAP: Mutex<FontMap> = Mutex::new(FontMap {
        font_map_light: vec![0_u8; FONT_SIZE],
        font_map_dark: vec![0_u8; FONT_SIZE],
        current_map_light: true,
    });
}

const LAYER_WIDTH: u32 = 512 * TILE_WIDTH;
const LAYER_HEIGHT: u32 = 256 * TILE_HEIGHT;
const LAYER_PITCH: usize = LAYER_WIDTH as usize * 4;
const LAYER_SIZE: usize = LAYER_PITCH * LAYER_HEIGHT as usize;
const LAYER_RECT: (i32, i32, u32, u32) = (0, 0, LAYER_WIDTH, LAYER_HEIGHT);
const AMOUNT_LAYER: usize = 6;
const LAYER_TYPES: [Layer; AMOUNT_LAYER] = [
    Layer::Tile,
    Layer::Entity,
    Layer::Tile,
    Layer::Entity,
    Layer::Text,
    Layer::Pixel,
];

#[derive(Eq, PartialEq)]
enum Layer {
    Tile,
    Entity,
    Text,
    Pixel,
}

struct Layers {
    layers: [Vec<u8>; AMOUNT_LAYER], // Pixel, Text, Entity (1), Tile (1), Entity (0), Tile (0).
    current_entity_layer: u32,
    current_tile_layer: u32,
    allow_pixel_layer: bool,
}

const SPRITE_WIDTH: u32 = 16;
const SPRITE_HEIGHT: u32 = 16;
const SPRITE_PITCH: usize = SPRITE_WIDTH as usize * 4;
const SPRITE_SIZE: usize = SPRITE_PITCH * SPRITE_HEIGHT as usize;
const SPRITE_MAP_SIZE: usize = SPRITE_SIZE * AMOUNT_SPRITES;
const AMOUNT_SPRITES: usize = MAX_ENTITIES as usize;
const MAX_ENTITIES: u32 = 256;

struct Entities {
    sprite_map: Vec<u8>,
    e_map: HashMap<u8, (u16, u16, u8, bool)> // x, y, sprite num, render
}

const TILE_WIDTH: u32 = 16;
const TILE_HEIGHT: u32 = 16;
const TILE_PITCH: usize = TILE_WIDTH as usize * 4;
const TILE_SIZE: usize = TILE_PITCH * TILE_HEIGHT as usize;
const AMOUNT_TILES: usize = 16;

const TILE_MAP_SIZE: usize = TILE_SIZE * AMOUNT_TILES;
const AMOUNT_TILE_MAP: usize = 4;

struct TileMaps {
    tile_maps: [Vec<u8>; AMOUNT_TILE_MAP],
    current_map: u32,
    current_tile: u32,
}

const FONT_WIDTH: u32 = 128;
const FONT_HEIGHT: u32 = 64;
const FONT_PITCH: usize = FONT_WIDTH as usize * 4;
const FONT_SIZE: usize = FONT_PITCH * FONT_HEIGHT as usize;

const LETTER_WIDTH: u32 = 8;
const LETTER_HEIGHT: u32 = 8;
const LETTER_PITCH: usize = LETTER_WIDTH as usize * 4;
const LETTER_SIZE: usize = LETTER_PITCH * LETTER_HEIGHT as usize;
const AMOUNT_LETTERS: usize = 128;

struct FontMap {
    font_map_light: Vec<u8>,
    font_map_dark: Vec<u8>,
    current_map_light: bool,
}

// All tile maps are loaded before program start.
// Tiles should be handled in another part of the program.
// pub fn load_tile_maps(maps: &[&[u8; TILE_MAP_SIZE]; AMOUNT_TILE_MAPS]) {}

// All font maps are loaded before program start.
// pub fn load_font_maps(light: &[u8; FONT_SIZE], dark: &[u8; FONT_SIZE]) {}

// WARN: This function bypasses a lot of checks!!!
pub fn load_image_into_layer(layer: usize, img: &[u8], width: usize, height: usize) {
    if layer < AMOUNT_LAYER {
        let mut layers = LAYERS.lock().unwrap();
        let buffer = layers.layers.get_mut(layer).unwrap();

        for i in 0..height {
            buffer.splice((i * LAYER_WIDTH as usize * 4)..(i * (LAYER_WIDTH as usize) * 4 + width * 4), img[(i * width * 4)..(i * width * 4 + width * 4)].iter().cloned());
        }
    }
    else {
        error!("load_image_into_layer received a layer value greater then the amount of layers!");
    }
}

pub fn init_textures(tex_creator: &TextureCreator<WindowContext>) -> Result<Vec<Texture>, String> {
    let mut textures = Vec::<Texture>::new();
    for _i in 0..AMOUNT_LAYER {
        let mut tex = tex_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, LAYER_WIDTH, LAYER_HEIGHT)
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
    let viewport= canvas.viewport();
    let viewport = Rect::new(i32::abs(viewport.x()), i32::abs(viewport.y()), viewport.width(), viewport.height());
    debug!("{:?}", viewport);
    for tex in textures.iter() {
        canvas.copy(&tex, Some(viewport), None)?;
    }

    canvas.present();

    Ok(())
}

fn build_textures(textures: &mut Vec<Texture>) -> Result<(), String> {
    if textures.len() == AMOUNT_LAYER {
        let layers = LAYERS.lock().unwrap();

        for (i, layer) in layers.layers.iter().enumerate() {
            if LAYER_TYPES[i] != Layer::Pixel || (i == AMOUNT_LAYER - 1 && layers.allow_pixel_layer)
            {
                proc_texture(i, textures.borrow_mut(), layer)?;
            }
        }
    }
    else {
        return Err("Wrong number of texture layers were passed to the draw function!".to_string());
    }

    Ok(())
}

fn proc_texture(index: usize, textures: &mut Vec<Texture>, buffer: &[u8]) -> Result<(), String> {
    match textures.get_mut(index) {
        None => return Err("Textures vector is missing a layer!".to_string()),
        Some(tex) => {
            tex.update(Rect::from(LAYER_RECT), buffer, LAYER_PITCH)
                .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

pub mod colors {
    pub const WHITE: u16 = create_color(15, 15, 15, 15);
    pub const BLACK: u16 = create_color(0, 0, 0, 15);

    pub const fn create_color(r: u8, g: u8, b: u8, a: u8) -> u16 {
        (((r & 15) as u16) << 12)
            | (((g & 15) as u16) << 8)
            | (((b & 15) as u16) << 4)
            | ((a & 15) as u16)
    }

    pub const fn map_color(color: u16) -> (u8, u8, u8, u8) {
        (
            (15 & ((color >> 12) as u8)) * 17,
            (15 & ((color >> 8) as u8)) * 17,
            (15 & ((color >> 4) as u8)) * 17,
            ((15 & color) as u8) * 17,
        )
    }

    pub fn map_color_vec(color: u16) -> Vec<u8> {
        let (r, g, b, a) = map_color(color);
        vec![r, g, b, a]
    }
}

pub mod commands {
    use crate::render::*;
    use rhai::plugin::*;

    // Color Stuff
    #[export_fn]
    pub fn create_color(r: u8, g: u8, b: u8, a: u8) -> u16 {
        colors::create_color(r, g, b, a)
    }

    // Layers
    #[export_fn]
    pub fn switch_entity_layer() -> u32 {
        let mut layers = LAYERS.lock().unwrap();
        layers.current_entity_layer = match layers.current_entity_layer {
            0 => 1,
            _ => 0,
        };
        layers.current_entity_layer
    }

    #[export_fn]
    pub fn switch_tile_layer() -> u32 {
        let mut layers = LAYERS.lock().unwrap();
        layers.current_tile_layer = match layers.current_tile_layer {
            0 => 1,
            _ => 0,
        };
        layers.current_tile_layer
    }

    #[export_fn]
    pub fn toggle_pixel_layer() -> bool {
        let mut layers = LAYERS.lock().unwrap();
        layers.allow_pixel_layer = !layers.allow_pixel_layer;

        debug!("Pixel Layer: {}", layers.allow_pixel_layer);

        layers.allow_pixel_layer
    }

    // Entities
    /*#[export_fn]
    pub fn create_entity(x: u16, y: u16, sprite: u8, visible: bool) -> u64 {
        if x < (TILE_WIDTH * )

        let mut entities = ENTITIES.lock().unwrap();

    }*/

    // struct Entities {
    //     sprite_map: Vec<u8>,
    //     e_map: HashMap<u8, (u16, u16, u8, bool)> // x, y, sprite num, render
    // }


    // Tile Stuff

    // Entity

    // Pixel Stuff

    //TODO Way to bulk draw pixels.
    #[export_fn]
    pub fn draw_pixel(x: u32, y: u32, color: u16) {
        let mut layers = LAYERS.lock().unwrap();
        if !layers.allow_pixel_layer {
            return;
        }

        let buffer = layers.layers.get_mut(AMOUNT_LAYER - 1).unwrap();

        let offset = (x * 4) as usize + (y as usize * LAYER_PITCH);
        let rgba: Vec<u8> = colors::map_color_vec(color);

        // debug!("Gamer: {:?}", rgba);

        buffer.splice((offset)..(offset + 4), rgba.iter().cloned());
    }
}
