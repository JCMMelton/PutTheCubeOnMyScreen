#version 330 core

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform float t1;
uniform float t2;

in vec3 position;
in vec3 normal;
out vec3 FragPos;
out vec3 Normal;

layout(location = 1) in vec3 colors;

float hypot(float p1, float p2) {
    return sqrt(pow(p1, 2.0) + pow(p2, 2.0));
}

void main() {
    vec3 n_pos = vec3(position.xy, 0.01*sin(50.0*hypot(position.x, position.y)-t1) );
    gl_Position = projection * view * model * vec4(n_pos, 1.0);
    FragPos = vec3(model * vec4(n_pos, 1.0));
    Normal = mat3(transpose(inverse(model))) * normal;
}