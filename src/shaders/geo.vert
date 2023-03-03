#version 330 core
in vec3 pos;
in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 Normal;
out vec3 FragPos;

void main() {
    FragPos = vec3(model * vec4(pos, 1.0));
    Normal = normal;
    gl_Position = projection * view * vec4(FragPos, 1.0);
}