#version 450

//layout(location = 0) in vec3 a_position;
//layout(location = 1) in vec2 a_tex_coords;

layout(location = 0) in uint index;
layout(location = 1) in ivec2 position;
layout(location = 2) in uvec4 color;

//layout(location = 0) out vec2 v_tex_coords;
layout(location = 0) out vec2 tex_coords;
layout(location = 1) out uint texture;
layout(location = 2) out vec4 out_color;

layout(set = 1, binding = 0)
uniform Uniforms {
    mat4 u_view_proj;
};

// Render
const uint color_scaler = 255;
const uint screen_size = 256;

// Tile Maps
//const uint size = 64;
//const uint tile_size = 16;
//const uint tile_amount = 16;

// Sprite Maps

void main() {
    if(index < 256) {
        // Text
    }
    else if(index < 512) {
        // Tiles
    }
    else if(index < 768) {
        // Sprites
    }
    else {
        // Pixel
    }


    vec3 a_position = vec3(0.0, 0.0, 0.0); //TODO Calculate this!
//    tex_coords = vec2(0.0, 0.0); //TODO Calculate this!

    // Camera or something
    //v_tex_coords = a_tex_coords;
    gl_Position = u_view_proj * vec4(a_position, 1.0);
}