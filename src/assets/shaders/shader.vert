#version 450

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec2 a_tex_coords;

layout(location = 0) out vec2 v_tex_coords;

// Fonts
layout(set = 3, binding = 0) uniform uint text_black[1024]; // Black Text Map
layout(set = 3, binding = 1) uniform uint text_white[1024]; // White Text Map

// Tile Maps
layout(set = 4, binding = 0) uniform uint tile_1[256]; // Tile Map 1
layout(set = 4, binding = 1) uniform uint tile_2[256]; // Tile Map 2
layout(set = 4, binding = 2) uniform uint tile_3[256]; // Tile Map 3
layout(set = 4, binding = 3) uniform uint tile_4[256]; // Tile Map 4

// Sprite Maps
layout(set = 5, binding = 0) uniform vec3 sprite_1[64]; // Sprite Map 1 (x, y, sprite index)
layout(set = 5, binding = 1) uniform vec3 sprite_2[64]; // Sprite Map 2 (x, y, sprite index)

//TODO The pixel layer should do its own thing.

// Camera
layout(set = 6, binding = 0)
uniform Uniforms {
    mat4 u_view_proj;
};

void main() {
    v_tex_coords = a_tex_coords;
    gl_Position = u_view_proj * vec4(a_position, 1.0);
}
