# Snow64
[![Rust Build and Test](https://github.com/harmless-tech/snow64/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/harmless-tech/snow64/actions/workflows/rust.yml)

Snow64 is an experimental fantasy console.

# Features (Planned)

- 16-bit color (R - 4 bits, G - 4 bits, B - 4 bits, A - 4 bits) (u16)
- 256 x 256 screen resolution. (Can be scaled up) (Maybe a 512 x 256 option)
- 8 x 8 fonts with their own top-ish layer.
- 16 x 16 sprites/tiles with 4 layers. (Entities, Tiles, Entities, Tiles)
- Direct pixel draw mode. (On its own top layer, a 6th layer) -- Maybe???
- 512 tile x 256 tile maps. (This is loaded onto the Tile layers)
- Max of 64 sprites per layer.
- 2 font maps with 128 chars each.
- ~~4 tile maps with 16 tiles each.~~ 1 tile map with 256 tiles.
- ~~2 sprite maps with 16 sprites each.~~ 1 sprite map with 256 sprites.
- Scripting with rhai?, wren?, and/or typescript?.
- Some kind of sound.
- Some kind of cart size. (Carts will have their own format using snowbinary)
- Its own shell, with a basic file system.
- Its own pixel art program, code editor, and build system (flake).
