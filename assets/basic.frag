#version 330 core

uniform vec2 window_size;
out vec4 FragColor;
in vec3 ourColor;

void main() {
    vec2 uv = 2.0 * (gl_FragCoord.xy/window_size) - 1.0;

    FragColor = vec4(ourColor, 1.0);
}