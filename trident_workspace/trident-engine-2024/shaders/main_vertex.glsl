#version 450 core

layout (location = 0) in vec3 vertex_positions;
layout (location = 1) in vec3 vertex_colors;
layout (location = 2) in vec2 texture_coords;

out vec3 output_color;
out vec2 out_tex_coords;

void main() {
    gl_Position = vec4(vertex_positions, 1.0);
    output_color = vertex_colors;
    out_tex_coords = texture_coords;
}