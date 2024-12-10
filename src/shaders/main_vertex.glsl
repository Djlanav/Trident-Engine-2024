#version 450 core

layout (location = 0) in vec3 vertex_positions;

out vec3 output_color;

void main() {
    gl_Position = vec4(vertex_positions, 1.0);
    output_color = vec3(1.0, 0.0, 0.0);
}