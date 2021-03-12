#version 330

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 i_texture_coords;

out vec4 vertex_color;
out vec2 texture_coords;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(position, 1.0);
    vertex_color = vec4(color, 1.0);
    texture_coords = i_texture_coords;
}