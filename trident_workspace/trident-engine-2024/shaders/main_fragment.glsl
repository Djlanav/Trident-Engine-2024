#version 450 core

in vec3 output_color;
in vec2 out_tex_coords;

out vec4 final_color;

// Uniforms
uniform vec3 u_Color;
uniform sampler2D some_texture;

void main() {
    final_color = texture(some_texture, out_tex_coords) * vec4(u_Color, 1.0);
}