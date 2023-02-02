#version 100

attribute vec2 pos;
attribute vec2 inst_pos;
attribute vec2 inst_scale;

void main() {
    gl_Position = vec4(inst_scale.x * pos.x + inst_pos.x, inst_scale.y * pos.y + inst_pos.y, 0.0, 1.0);
}