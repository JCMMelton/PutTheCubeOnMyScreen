#version 330 core

uniform mat4 transform;

in vec3 position;
out vec3 ourColor;

layout(location = 1) in vec3 colors;

void main() {
    gl_Position = transform * vec4(position, 1.0);
    ourColor = gl_Position.xyz;
}