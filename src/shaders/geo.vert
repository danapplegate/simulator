#version 330 core

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 inst_pos;
layout(location = 2) in vec3 inst_scale;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(inst_scale.xyz * pos.xyz + inst_pos.xyz, 1.0);
}