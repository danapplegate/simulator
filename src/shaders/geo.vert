#version 330 core
in vec3 pos;
in vec3 normal;
in vec3 inst_pos;
in vec3 inst_scale;

uniform mat4 mvp;

out vec3 Normal;
out vec3 FragPos;

void main() {
    FragPos = inst_scale.xyz * pos.xyz + inst_pos.xyz;
    gl_Position = mvp * vec4(FragPos, 1.0);
    Normal = normal;
}