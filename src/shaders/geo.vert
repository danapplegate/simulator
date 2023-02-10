#version 100

attribute vec3 pos;
attribute vec3 inst_pos;
attribute vec3 inst_scale;

void main() {
    gl_Position = vec4(inst_scale.x * pos.x + inst_pos.x, inst_scale.y * pos.y + inst_pos.y, 0.0, 1.0);
}