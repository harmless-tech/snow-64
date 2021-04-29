#version 450

//layout(location = 0) in vec2 v_tex_coords;

layout(location = 0) in vec2 tex_coords;
layout(location = 1) in uint texture;
layout(location = 2) in vec4 color;

layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D text_t_diffuse[2]; // Black and White Text Maps
layout(set = 0, binding = 1) uniform sampler text_s_diffuse;

layout(set = 1, binding = 0) uniform texture2D tile_t_diffuse[1]; // One Tile Maps
layout(set = 1, binding = 1) uniform sampler tile_s_diffuse;

layout(set = 2, binding = 0) uniform texture2D sprite_t_diffuse[1]; // One Sprite Maps
layout(set = 2, binding = 1) uniform sampler sprite_s_diffuse;

//layout(set = 0, binding = 0) uniform texture2D t_diffuse[5];
//layout(set = 0, binding = 1) uniform sampler s_diffuse;

void main() {
    if(texture < 2) {
        // Text Maps
        f_color = texture(sampler2D(text_t_diffuse[texture], text_s_diffuse), tex_coords);
    }
    else if(texture < 3) {
        // Tile Maps
        f_color = texture(sampler2D(tile_t_diffuse[texture - 2], tile_s_diffuse), tex_coords);
    }
    else if(texture < 4) {
        // Sprite Maps
        f_color = texture(sampler2D(sprite_t_diffuse[texture - 3], sprite_s_diffuse), tex_coords);
    }
    else {
        // Pixel Layer (Default)
        f_color = color;
    }

//    vec4[5] t = {
//        texture(sampler2D(t_diffuse[0], s_diffuse), v_tex_coords),
//        texture(sampler2D(t_diffuse[1], s_diffuse), v_tex_coords),
//        texture(sampler2D(t_diffuse[2], s_diffuse), v_tex_coords),
//        texture(sampler2D(t_diffuse[3], s_diffuse), v_tex_coords),
//        texture(sampler2D(t_diffuse[4], s_diffuse), v_tex_coords)
//    };

    //f_color = mix(mix(mix(mix(t[0], t[1], t[1].a), t[2], t[2].a), t[3], t[3].a), t[4], t[4].a);
//    f_color = t[v_uint];
}