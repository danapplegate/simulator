#version 330 core

in vec3 Normal;
in vec3 FragPos;

out vec4 FragColor;

uniform vec3 light_color;
uniform vec3 light_pos;

void main() {
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * light_color;
    vec3 objectColor = vec3(0.0, 0.7, 0.7);

    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(light_pos - FragPos);

    float diffStrength = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diffStrength * light_color;

    FragColor = vec4((ambient + diffuse) * objectColor, 1.0);
}