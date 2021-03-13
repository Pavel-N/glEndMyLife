#version 330

in vec4 vertex_color;
in vec2 texture_coords;
in vec3 pos_color;

out vec4 fragment_color;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main() {
    fragment_color = vec4(pos_color, 1.0);
    //fragment_color = texture(texture1, texture_coords);
    //fragment_color = mix(texture(texture1, texture_coords), texture(texture2, texture_coords), 0.5);
}