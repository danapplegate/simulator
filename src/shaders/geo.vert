#version 330 core
in vec3 pos;
in vec3 normal;
in vec2 tex_coord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 Normal;
out vec3 FragPos;
out vec2 TexCoord;

void main() {
    FragPos = vec3(model * vec4(pos, 1.0));
    Normal = normal;
    gl_Position = projection * view * vec4(FragPos, 1.0);
    TexCoord = tex_coord;
}