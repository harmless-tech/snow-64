#version 450

layout(location = 0) in vec2 v_tex_coords;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse[5];
layout(set = 0, binding = 1) uniform sampler s_diffuse;
void main() {
    vec4[5] t = {
        texture(sampler2D(t_diffuse[0], s_diffuse), v_tex_coords),
        texture(sampler2D(t_diffuse[1], s_diffuse), v_tex_coords),
        texture(sampler2D(t_diffuse[2], s_diffuse), v_tex_coords),
        texture(sampler2D(t_diffuse[3], s_diffuse), v_tex_coords),
        texture(sampler2D(t_diffuse[4], s_diffuse), v_tex_coords)
    };

    f_color = mix(mix(mix(mix(t[0], t[1], t[1].a), t[2], t[2].a), t[3], t[3].a), t[4], t[4].a);
}
