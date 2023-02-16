#version 100

attribute vec3 pos;
attribute vec3 inst_pos;
attribute vec3 inst_scale;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(inst_scale.xyz * pos.xyz + inst_pos.xyz, 1.0);
}