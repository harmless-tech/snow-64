#version 450

layout(location = 0) in vec2 v_tex_coords;
layout(location = 0) out vec4 f_color; // This is the last thing. "returns"

// Fonts
layout(set = 0, binding = 0) uniform texture2D text_diffuse[2]; // Two Text Maps
layout(set = 0, binding = 1) uniform sampler s_text_diffuse;

// Tile Maps
layout(set = 1, binding = 0) uniform texture2D tile_diffuse[4]; // 4 Tile Maps
layout(set = 1, binding = 1) uniform sampler s_tile_diffuse;

// Sprite Maps
layout(set = 2, binding = 0) uniform texture2D sprite_diffuse[2]; // 2 Sprite Maps
layout(set = 2, binding = 1) uniform sampler s_sprite_diffuse;

//TODO The pixel layer should do its own thing.
// Pixel Layer
//layout(set = 2, binding = 0) uniform texture2D t_diffuse[4];
//layout(set = 2, binding = 1) uniform sampler s_diffuse;

void main() {
    /*vec4[5] t = {
        texture(sampler2D(t_diffuse[0], s_diffuse), v_tex_coords),
        texture(sampler2D(t_diffuse[1], s_diffuse), v_tex_coords),
        texture(sampler2D(t_diffuse[2], s_diffuse), v_tex_coords),
        texture(sampler2D(t_diffuse[3], s_diffuse), v_tex_coords),
        texture(sampler2D(t_diffuse[4], s_diffuse), v_tex_coords)
    };

    f_color = mix(mix(mix(mix(t[0], t[1], t[1].a), t[2], t[2].a), t[3], t[3].a), t[4], t[4].a);*/
}
