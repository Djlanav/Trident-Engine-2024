#version 450 core

layout (location = 0) in vec3 vertex_positions;
layout (location = 1) in vec3 vertex_colors;

// Uniforms
uniform vec3 u_Color;

out vec3 output_color;

void main() {
    gl_Position = vec4(vertex_positions, 1.0);
    output_color = vertex_colors;
}