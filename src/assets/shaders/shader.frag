#version 450

layout(location = 0) in vec2 v_tex_coords;
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t0_diffuse;
layout(set = 0, binding = 1) uniform sampler s0_diffuse;

layout(set = 1, binding = 0) uniform texture2D t1_diffuse;
layout(set = 1, binding = 1) uniform sampler s1_diffuse;

void main() {
    vec4 t0 = texture(sampler2D(t0_diffuse, s0_diffuse), v_tex_coords);
    vec4 t1 = texture(sampler2D(t1_diffuse, s1_diffuse), v_tex_coords);

    f_color = mix(t0, t1, t1.a);
}
